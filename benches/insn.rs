use criterion::{Criterion, criterion_group, criterion_main, black_box, BatchSize, BenchmarkId, Throughput};
use coffer::constants::insn::*;
use std::io::{Cursor, Seek, SeekFrom, BufReader, Read};
use coffer::insn::InstructionRead;
use std::fs::{read_dir, File};
use coffer::index::JClassIdx;
use std::thread::spawn;
use zip::ZipArchive;
use rand::{Rng, thread_rng};
use rand::distributions::{Uniform, Standard};
use rand_pcg::Pcg64;


fn bench_op(c: &mut Criterion) {
    let buf = [TABLESWITCH, 0, 0, 0, 12, 0, 0, 0, 10, 0, 0, 0, 12, 4, 6, 7, 4, 5, 6, 4, 7, 1, 35, 76, 34];
    c.bench_function("parse_tableswitch", |b| {
        b.iter_batched(|| {
            Cursor::new(buf)
        }, |mut c| c.read_insn(|_| {}), BatchSize::SmallInput)
    });

    let buf = [ATHROW];

    c.bench_function("parse_no_ext_bytes", |b| {
        b.iter_batched(|| {
            Cursor::new(buf)
        }, |mut c| c.read_insn(|_| {}), BatchSize::SmallInput)
    });
}

fn bench_jidx(c: &mut Criterion) -> coffer::error::Result<()> {
    // See benchmarking_licenses
    let urls = [
        "https://repo1.maven.org/maven2/com/google/guava/guava/30.0-jre/guava-30.0-jre.jar",
        "https://repo1.maven.org/maven2/com/squareup/okhttp3/okhttp/4.10.0-RC1/okhttp-4.10.0-RC1.jar",
        "https://repo1.maven.org/maven2/org/apache/spark/spark-core_2.11/2.4.7/spark-core_2.11-2.4.7.jar",
        "https://repo1.maven.org/maven2/com/google/zxing/core/3.4.1/core-3.4.1.jar"
    ];
    use std;
    let mut count = 31;
    let classes: Vec<(String, Vec<u8>)> = urls.iter().map(|&url|  {
        let thing = spawn(move || {
            let mut easy = curl::easy::Easy::new();
            let mut dst = vec![];
            easy.url(url).unwrap();
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                dst.extend_from_slice(data);
                Ok(data.len())
            }).unwrap();
            transfer.perform().unwrap();
            drop(transfer);
            dst
        });
        ZipArchive::new(Cursor::new(thing.join().unwrap())).unwrap()
    }).flat_map(|mut zip| {
        let len = zip.len();
        let rng: Pcg64 = rand::SeedableRng::seed_from_u64(1230984 + count);
        count *= 31;
        let uni = Uniform::new(0, len);
        let mut iter = rng.sample_iter(uni);
        (0..10).map(move |_| {
            loop {
                let mut entry = zip.by_index(iter.next().unwrap()).unwrap();
                if entry.is_file() && entry.name().ends_with(".class") && !entry.name().contains('$') /* Avoid Nested Classes */ {
                    let mut vec = vec![];
                    entry.read_to_end(&mut vec);
                    break (entry.name().to_string(), vec);
                }
            }
        }).collect::<Vec<(String, Vec<u8>)>>()
    }).collect();

    let mut group = c.benchmark_group("bench_jidx");
    for (name, bytes) in classes {
        group.throughput(Throughput::Bytes(bytes.len() as u64));
        group.bench_with_input(BenchmarkId::from_parameter(name), &bytes, |c, buf| {
            c.iter_batched(|| Cursor::new(buf), |mut c | JClassIdx::try_from(&mut c), BatchSize::SmallInput)
        });
    }
    /*for d in read_dir("out")? {
        let d = d?;
        let path = d.path();
        let path_ref = path.as_path();
        if !path.is_file() { continue; }
        if let Some(ext) = path.extension() {
            if ext == "class" {
                let mut file = File::open(path_ref)?;
                let meta = file.metadata()?;
                let mut bytes = vec![];
                file.read_to_end(&mut bytes);
                let byte_len = meta.len();
                let filename = path_ref.file_name().unwrap().to_str().unwrap();
                group.throughput(Throughput::Bytes(byte_len));
                group.bench_with_input(BenchmarkId::from_parameter(filename), &bytes, |c, buf| {
                    c.iter_batched(|| {
                        Cursor::new(buf)
                    }, |mut c| JClassIdx::try_from(&mut c), BatchSize::SmallInput)
                });
            }
        }
    }*/

    Ok(())
}

criterion_group!(benches, bench_op, bench_jidx);
criterion_main!(benches);