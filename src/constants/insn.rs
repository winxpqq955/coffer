/*
    This file is part of Coffer.

    Coffer is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Coffer is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with Coffer. (LICENSE.md)  If not, see <https://www.gnu.org/licenses/>.
*/
#![allow(dead_code)]
pub const NOP: u8 = 0x00;
pub const ACONST_NULL: u8 = 0x01;
pub const ICONST_M1: u8 = 0x02;
pub const ICONST_0: u8 = 0x03;
pub const ICONST_1: u8 = 0x04;
pub const ICONST_2: u8 = 0x05;
pub const ICONST_3: u8 = 0x06;
pub const ICONST_4: u8 = 0x07;
pub const ICONST_5: u8 = 0x08;
pub const LCONST_0: u8 = 0x09;
pub const LCONST_1: u8 = 0x0A;
pub const FCONST_0: u8 = 0x0B;
pub const FCONST_1: u8 = 0x0C;
pub const FCONST_2: u8 = 0x0D;
pub const DCONST_0: u8 = 0x0E;
pub const DCONST_1: u8 = 0x0F;
pub const BIPUSH: u8 = 0x10;
pub const SIPUSH: u8 = 0x11;
pub const LDC: u8 = 0x12;
pub const LDC_W: u8 = 0x13;
pub const LDC2_W: u8 = 0x14;
pub const ILOAD: u8 = 0x15;
pub const LLOAD: u8 = 0x16;
pub const FLOAD: u8 = 0x17;
pub const DLOAD: u8 = 0x18;
pub const ALOAD: u8 = 0x19;
pub const ILOAD_0: u8 = 0x1A;
pub const ILOAD_1: u8 = 0x1B;
pub const ILOAD_2: u8 = 0x1C;
pub const ILOAD_3: u8 = 0x1D;
pub const LLOAD_0: u8 = 0x1E;
pub const LLOAD_1: u8 = 0x1F;
pub const LLOAD_2: u8 = 0x20;
pub const LLOAD_3: u8 = 0x21;
pub const FLOAD_0: u8 = 0x22;
pub const FLOAD_1: u8 = 0x23;
pub const FLOAD_2: u8 = 0x24;
pub const FLOAD_3: u8 = 0x25;
pub const DLOAD_0: u8 = 0x26;
pub const DLOAD_1: u8 = 0x27;
pub const DLOAD_2: u8 = 0x28;
pub const DLOAD_3: u8 = 0x29;
pub const ALOAD_0: u8 = 0x2A;
pub const ALOAD_1: u8 = 0x2B;
pub const ALOAD_2: u8 = 0x2C;
pub const ALOAD_3: u8 = 0x2D;
pub const IALOAD: u8 = 0x2E;
pub const LALOAD: u8 = 0x2F;
pub const FALOAD: u8 = 0x30;
pub const DALOAD: u8 = 0x31;
pub const AALOAD: u8 = 0x32;
pub const BALOAD: u8 = 0x33;
pub const CALOAD: u8 = 0x34;
pub const SALOAD: u8 = 0x35;
pub const ISTORE: u8 = 0x36;
pub const LSTORE: u8 = 0x37;
pub const FSTORE: u8 = 0x38;
pub const DSTORE: u8 = 0x39;
pub const ASTORE: u8 = 0x3A;
pub const ISTORE_0: u8 = 0x3B;
pub const ISTORE_1: u8 = 0x3C;
pub const ISTORE_2: u8 = 0x3D;
pub const ISTORE_3: u8 = 0x3E;
pub const LSTORE_0: u8 = 0x3F;
pub const LSTORE_1: u8 = 0x40;
pub const LSTORE_2: u8 = 0x41;
pub const LSTORE_3: u8 = 0x42;
pub const FSTORE_0: u8 = 0x43;
pub const FSTORE_1: u8 = 0x44;
pub const FSTORE_2: u8 = 0x45;
pub const FSTORE_3: u8 = 0x46;
pub const DSTORE_0: u8 = 0x47;
pub const DSTORE_1: u8 = 0x48;
pub const DSTORE_2: u8 = 0x49;
pub const DSTORE_3: u8 = 0x4A;
pub const ASTORE_0: u8 = 0x4B;
pub const ASTORE_1: u8 = 0x4C;
pub const ASTORE_2: u8 = 0x4D;
pub const ASTORE_3: u8 = 0x4E;
pub const IASTORE: u8 = 0x4F;
pub const LASTORE: u8 = 0x50;
pub const FASTORE: u8 = 0x51;
pub const DASTORE: u8 = 0x52;
pub const AASTORE: u8 = 0x53;
pub const BASTORE: u8 = 0x54;
pub const CASTORE: u8 = 0x55;
pub const SASTORE: u8 = 0x56;
pub const POP: u8 = 0x57;
pub const POP2: u8 = 0x58;
pub const DUP: u8 = 0x59;
pub const DUP_X1: u8 = 0x5A;
pub const DUP_X2: u8 = 0x5B;
pub const DUP2: u8 = 0x5C;
pub const DUP2_X1: u8 = 0x5D;
pub const DUP2_X2: u8 = 0x5E;
pub const SWAP: u8 = 0x5F;
pub const IADD: u8 = 0x60;
pub const LADD: u8 = 0x61;
pub const FADD: u8 = 0x62;
pub const DADD: u8 = 0x63;
pub const ISUB: u8 = 0x64;
pub const LSUB: u8 = 0x65;
pub const FSUB: u8 = 0x66;
pub const DSUB: u8 = 0x67;
pub const IMUL: u8 = 0x68;
pub const LMUL: u8 = 0x69;
pub const FMUL: u8 = 0x6A;
pub const DMUL: u8 = 0x6B;
pub const IDIV: u8 = 0x6C;
pub const LDIV: u8 = 0x6D;
pub const FDIV: u8 = 0x6E;
pub const DDIV: u8 = 0x6F;
pub const IREM: u8 = 0x70;
pub const LREM: u8 = 0x71;
pub const FREM: u8 = 0x72;
pub const DREM: u8 = 0x73;
pub const INEG: u8 = 0x74;
pub const LNEG: u8 = 0x75;
pub const FNEG: u8 = 0x76;
pub const DNEG: u8 = 0x77;
pub const ISHL: u8 = 0x78;
pub const LSHL: u8 = 0x79;
pub const ISHR: u8 = 0x7A;
pub const LSHR: u8 = 0x7B;
pub const IUSHR: u8 = 0x7C;
pub const LUSHR: u8 = 0x7D;
pub const IAND: u8 = 0x7E;
pub const LAND: u8 = 0x7F;
pub const IOR: u8 = 0x80;
pub const LOR: u8 = 0x81;
pub const IXOR: u8 = 0x82;
pub const LXOR: u8 = 0x83;
pub const IINC: u8 = 0x84;
pub const I2L: u8 = 0x85;
pub const I2F: u8 = 0x86;
pub const I2D: u8 = 0x87;
pub const L2I: u8 = 0x88;
pub const L2F: u8 = 0x89;
pub const L2D: u8 = 0x8A;
pub const F2I: u8 = 0x8B;
pub const F2L: u8 = 0x8C;
pub const F2D: u8 = 0x8D;
pub const D2I: u8 = 0x8E;
pub const D2L: u8 = 0x8F;
pub const D2F: u8 = 0x90;
pub const I2B: u8 = 0x91;
pub const I2C: u8 = 0x92;
pub const I2S: u8 = 0x93;
pub const LCMP: u8 = 0x94;
pub const FCMPL: u8 = 0x95;
pub const FCMPG: u8 = 0x96;
pub const DCMPL: u8 = 0x97;
pub const DCMPG: u8 = 0x98;
pub const IFEQ: u8 = 0x99;
pub const IFNE: u8 = 0x9A;
pub const IFLT: u8 = 0x9B;
pub const IFGE: u8 = 0x9C;
pub const IFGT: u8 = 0x9D;
pub const IFLE: u8 = 0x9E;
pub const IFICMPEQ: u8 = 0x9F;
pub const IFICMPNE: u8 = 0xA0;
pub const IFICMPLT: u8 = 0xA1;
pub const IFICMPGE: u8 = 0xA2;
pub const IFICMPGT: u8 = 0xA3;
pub const IFICMPLE: u8 = 0xA4;
pub const IFACMPEQ: u8 = 0xA5;
pub const IFACMPNE: u8 = 0xA6;
pub const GOTO: u8 = 0xA7;
pub const JSR: u8 = 0xA8;
pub const RET: u8 = 0xA9;
pub const TABLESWITCH: u8 = 0xAA;
pub const LOOKUPSWITCH: u8 = 0xAB;
pub const IRETURN: u8 = 0xAC;
pub const LRETURN: u8 = 0xAD;
pub const FRETURN: u8 = 0xAE;
pub const DRETURN: u8 = 0xAF;
pub const ARETURN: u8 = 0xB0;
pub const RETURN: u8 = 0xB1;
pub const GETSTATIC: u8 = 0xB2;
pub const PUTSTATIC: u8 = 0xB3;
pub const GETFIELD: u8 = 0xB4;
pub const PUTFIELD: u8 = 0xB5;
pub const INVOKEVIRTUAL: u8 = 0xB6;
pub const INVOKESPECIAL: u8 = 0xB7;
pub const INVOKESTATIC: u8 = 0xB8;
pub const INVOKEINTERFACE: u8 = 0xB9;
pub const INVOKEDYNAMIC: u8 = 0xBA;
pub const NEW: u8 = 0xBB;
pub const NEWARRAY: u8 = 0xBC;
pub const ANEWARRAY: u8 = 0xBD;
pub const ARRAYLENGTH: u8 = 0xBE;
pub const ATHROW: u8 = 0xBF;
pub const CHECKCAST: u8 = 0xC0;
pub const INSTANCEOF: u8 = 0xC1;
pub const MONITORENTER: u8 = 0xC2;
pub const MONITOREXIT: u8 = 0xC3;
pub const WIDE: u8 = 0xC4;
pub const MULTIANEWARRAY: u8 = 0xC5;
pub const IFNULL: u8 = 0xC6;
pub const IFNONNULL: u8 = 0xC7;
pub const GOTO_W: u8 = 0xC8;
pub const JSR_W: u8 = 0xC9;