use crate::code::Arg;

pub enum AddressingMode {
    Short(u8),
    Long {
        low: u8,
        high: u8,
    }
}
#[derive(Debug)]
pub enum Instruction {
    Return,
    Returnr(Arg),
    Call(Arg),
    CallR(Arg, Arg, Arg, Arg, Arg),
    CallT(Arg),
    CallGr(Arg, Arg, Arg, Arg, Arg, Arg),
    Jump(Arg),
    Jumpif(Arg, Arg),
    SwitchV2(Arg, Arg, Arg),
    SwitchV3(Arg, Arg, Arg, Arg),
    SwitchVn(Arg, Arg),
    CallValue(Arg),
    Push(Arg),
    Dupa,
    Dup(Arg),
    Pop(Arg),
    Inca,
    Inc(Arg),
    Deca,
    Dec(Arg),
    Add(Arg, Arg, Arg),
    Sub(Arg, Arg, Arg),
    Mul(Arg, Arg, Arg),
    Div(Arg, Arg, Arg),
    Mod(Arg, Arg, Arg),
    Pow(Arg, Arg, Arg),
    Store(Arg, Arg),
    Sha3(Arg, Arg),
    Sha256(Arg, Arg),
    Blake2B(Arg, Arg),
    Lt(Arg, Arg, Arg),
    Gt(Arg, Arg, Arg),
    Eq(Arg, Arg, Arg),
    Elt(Arg, Arg, Arg),
    Egt(Arg, Arg, Arg),
    Neq(Arg, Arg, Arg),
    And(Arg, Arg, Arg),
    Or(Arg, Arg, Arg),
    Not(Arg, Arg),
    Tuple(Arg, Arg),
    Element(Arg, Arg, Arg),
    Setelement(Arg, Arg, Arg, Arg),
    MapEmpty(Arg),
    MapLookup(Arg, Arg, Arg),
    MapLookupd(Arg, Arg, Arg, Arg),
    MapUpdate(Arg, Arg, Arg, Arg),
    MapDelete(Arg, Arg, Arg),
    MapMember(Arg, Arg, Arg),
    MapFromList(Arg, Arg),
    MapSize(Arg, Arg),
    MapToList(Arg, Arg),
    IsNil(Arg, Arg),
    Cons(Arg, Arg, Arg),
    Hd(Arg, Arg),
    Tl(Arg, Arg),
    Length(Arg, Arg),
    Nil(Arg),
    Append(Arg, Arg, Arg),
    StrJoin(Arg, Arg, Arg),
    IntToStr(Arg, Arg),
    AddrToStr(Arg, Arg),
    StrReverse(Arg, Arg),
    StrLength(Arg, Arg),
    BytesToInt(Arg, Arg),
    BytesToStr(Arg, Arg),
    BytesConcat(Arg, Arg, Arg),
    BytesSplit(Arg, Arg, Arg),
    IntToAddr(Arg, Arg),
    Variant(Arg, Arg, Arg, Arg),
    VariantTest(Arg, Arg, Arg),
    VariantElement(Arg, Arg, Arg),
    BitsNonea,
    BitsNone(Arg),
    BitsAlla,
    BitsAll(Arg),
    BitsAllN(Arg, Arg),
    BitsSet(Arg, Arg, Arg),
    BitsClear(Arg, Arg, Arg),
    BitsTest(Arg, Arg, Arg),
    BitsSum(Arg, Arg),
    BitsOr(Arg, Arg, Arg),
    BitsAnd(Arg, Arg, Arg),
    BitsDiff(Arg, Arg, Arg),
    Balance(Arg),
    Origin(Arg),
    Caller(Arg),
    Blockhash(Arg, Arg),
    Beneficiary(Arg),
    Timestamp(Arg),
    Generation(Arg),
    Microblock(Arg),
    Difficulty(Arg),
    Gaslimit(Arg),
    Gas(Arg),
    Address(Arg),
    Gasprice(Arg),
    Log0(Arg),
    Log1(Arg, Arg),
    Log2(Arg, Arg, Arg),
    Log3(Arg, Arg, Arg, Arg),
    Log4(Arg, Arg, Arg, Arg, Arg),
    Spend(Arg, Arg),
    OracleRegister(Arg, Arg, Arg, Arg, Arg, Arg, Arg),
    OracleQuery(Arg, Arg, Arg, Arg, Arg, Arg, Arg, Arg),
    OracleRespond(Arg, Arg, Arg, Arg, Arg, Arg),
    OracleExtend(Arg, Arg, Arg),
    OracleGetAnswer(Arg, Arg, Arg, Arg, Arg),
    OracleGetQuestion(Arg, Arg, Arg, Arg, Arg),
    OracleQueryFee(Arg, Arg),
    AensResolve(Arg, Arg, Arg, Arg),
    AensPreclaim(Arg, Arg, Arg),
    AensClaim(Arg, Arg, Arg, Arg, Arg),
    AensUpdate(Arg, Arg, Arg, Arg, Arg, Arg),
    AensTransfer(Arg, Arg, Arg, Arg),
    AensRevoke(Arg, Arg, Arg),
    BalanceOther(Arg, Arg),
    VerifySig(Arg, Arg, Arg, Arg),
    VerifySigSecp256K1(Arg, Arg, Arg, Arg),
    ContractToAddress(Arg, Arg),
    AuthTxHash(Arg),
    OracleCheck(Arg, Arg, Arg, Arg),
    OracleCheckQuery(Arg, Arg, Arg, Arg, Arg),
    IsOracle(Arg, Arg),
    IsContract(Arg, Arg),
    IsPayable(Arg, Arg),
    Creator(Arg),
    EcverifySecp256K1(Arg, Arg, Arg, Arg),
    EcrecoverSecp256K1(Arg, Arg, Arg),
    AddressToContract(Arg, Arg),
    Bls12_381G1Neg(Arg, Arg),
    Bls12_381G1Norm(Arg, Arg),
    Bls12_381G1Valid(Arg, Arg),
    Bls12_381G1IsZero(Arg, Arg),
    Bls12_381G1Add(Arg, Arg, Arg),
    Bls12_381G1Mul(Arg, Arg, Arg),
    Bls12_381G2Neg(Arg, Arg),
    Bls12_381G2Norm(Arg, Arg),
    Bls12_381G2Valid(Arg, Arg),
    Bls12_381G2IsZero(Arg, Arg),
    Bls12_381G2Add(Arg, Arg, Arg),
    Bls12_381G2Mul(Arg, Arg, Arg),
    Bls12_381GtInv(Arg, Arg),
    Bls12_381GtAdd(Arg, Arg, Arg),
    Bls12_381GtMul(Arg, Arg, Arg),
    Bls12_381GtPow(Arg, Arg, Arg),
    Bls12_381GtIsOne(Arg, Arg),
    Bls12_381Pairing(Arg, Arg, Arg),
    Bls12_381MillerLoop(Arg, Arg, Arg),
    Bls12_381FinalExp(Arg, Arg),
    Bls12_381IntToFr(Arg, Arg),
    Bls12_381IntToFp(Arg, Arg),
    Bls12_381FrToInt(Arg, Arg),
    Bls12_381FpToInt(Arg, Arg),
    AensLookup(Arg, Arg),
    OracleExpiry(Arg, Arg),
    AuthTx(Arg),
    StrToList(Arg, Arg),
    StrFromList(Arg, Arg),
    StrToUpper(Arg, Arg),
    StrToLower(Arg, Arg),
    CharToInt(Arg, Arg),
    CharFromInt(Arg, Arg),
    CallPgr(Arg, Arg, Arg, Arg, Arg, Arg, Arg),
    Create(Arg, Arg, Arg),
    Clone(Arg, Arg, Arg, Arg),
    CloneG(Arg, Arg, Arg, Arg, Arg),
    BytecodeHash(Arg, Arg),
    Fee(Arg),
    AddressToBytes(Arg, Arg),
    Poseidon(Arg, Arg, Arg),
    Mulmod(Arg, Arg, Arg, Arg),
    Band(Arg, Arg, Arg),
    Bor(Arg, Arg, Arg),
    Bxor(Arg, Arg, Arg),
    Bnot(Arg, Arg),
    Bsl(Arg, Arg, Arg),
    Bsr(Arg, Arg, Arg),
    BytesSplitAny(Arg, Arg, Arg),
    BytesSize(Arg, Arg),
    BytesToFixedSize(Arg, Arg, Arg),
    IntToBytes(Arg, Arg, Arg),
    StrToBytes(Arg, Arg),
    DbgLoc(Arg, Arg),
    DbgDef(Arg, Arg),
    DbgUndef(Arg, Arg),
    DbgContract(Arg),
    Deactivate,
    Abort(Arg),
    Exit(Arg),
    Nop,
}
impl Instruction {
    pub fn opcode(&self) -> u8 {
        use Instruction::*;
        match self {
            Return => 0x0,
            Returnr(_) => 0x1,
            Call(_) => 0x2,
            CallR(_, _, _, _, _) => 0x3,
            CallT(_) => 0x4,
            CallGr(_, _, _, _, _, _) => 0x5,
            Jump(_) => 0x6,
            Jumpif(_, _) => 0x7,
            SwitchV2(_, _, _) => 0x8,
            SwitchV3(_, _, _, _) => 0x9,
            SwitchVn(_, _) => 0xa,
            CallValue(_) => 0xb,
            Push(_) => 0xc,
            Dupa => 0xd,
            Dup(_) => 0xe,
            Pop(_) => 0xf,
            Inca => 0x10,
            Inc(_) => 0x11,
            Deca => 0x12,
            Dec(_) => 0x13,
            Add(_, _, _) => 0x14,
            Sub(_, _, _) => 0x15,
            Mul(_, _, _) => 0x16,
            Div(_, _, _) => 0x17,
            Mod(_, _, _) => 0x18,
            Pow(_, _, _) => 0x19,
            Store(_, _) => 0x1a,
            Sha3(_, _) => 0x1b,
            Sha256(_, _) => 0x1c,
            Blake2B(_, _) => 0x1d,
            Lt(_, _, _) => 0x1e,
            Gt(_, _, _) => 0x1f,
            Eq(_, _, _) => 0x20,
            Elt(_, _, _) => 0x21,
            Egt(_, _, _) => 0x22,
            Neq(_, _, _) => 0x23,
            And(_, _, _) => 0x24,
            Or(_, _, _) => 0x25,
            Not(_, _) => 0x26,
            Tuple(_, _) => 0x27,
            Element(_, _, _) => 0x28,
            Setelement(_, _, _, _) => 0x29,
            MapEmpty(_) => 0x2a,
            MapLookup(_, _, _) => 0x2b,
            MapLookupd(_, _, _, _) => 0x2c,
            MapUpdate(_, _, _, _) => 0x2d,
            MapDelete(_, _, _) => 0x2e,
            MapMember(_, _, _) => 0x2f,
            MapFromList(_, _) => 0x30,
            MapSize(_, _) => 0x31,
            MapToList(_, _) => 0x32,
            IsNil(_, _) => 0x33,
            Cons(_, _, _) => 0x34,
            Hd(_, _) => 0x35,
            Tl(_, _) => 0x36,
            Length(_, _) => 0x37,
            Nil(_) => 0x38,
            Append(_, _, _) => 0x39,
            StrJoin(_, _, _) => 0x3a,
            IntToStr(_, _) => 0x3b,
            AddrToStr(_, _) => 0x3c,
            StrReverse(_, _) => 0x3d,
            StrLength(_, _) => 0x3e,
            BytesToInt(_, _) => 0x3f,
            BytesToStr(_, _) => 0x40,
            BytesConcat(_, _, _) => 0x41,
            BytesSplit(_, _, _) => 0x42,
            IntToAddr(_, _) => 0x43,
            Variant(_, _, _, _) => 0x44,
            VariantTest(_, _, _) => 0x45,
            VariantElement(_, _, _) => 0x46,
            BitsNonea => 0x47,
            BitsNone(_) => 0x48,
            BitsAlla => 0x49,
            BitsAll(_) => 0x4a,
            BitsAllN(_, _) => 0x4b,
            BitsSet(_, _, _) => 0x4c,
            BitsClear(_, _, _) => 0x4d,
            BitsTest(_, _, _) => 0x4e,
            BitsSum(_, _) => 0x4f,
            BitsOr(_, _, _) => 0x50,
            BitsAnd(_, _, _) => 0x51,
            BitsDiff(_, _, _) => 0x52,
            Balance(_) => 0x53,
            Origin(_) => 0x54,
            Caller(_) => 0x55,
            Blockhash(_, _) => 0x56,
            Beneficiary(_) => 0x57,
            Timestamp(_) => 0x58,
            Generation(_) => 0x59,
            Microblock(_) => 0x5a,
            Difficulty(_) => 0x5b,
            Gaslimit(_) => 0x5c,
            Gas(_) => 0x5d,
            Address(_) => 0x5e,
            Gasprice(_) => 0x5f,
            Log0(_) => 0x60,
            Log1(_, _) => 0x61,
            Log2(_, _, _) => 0x62,
            Log3(_, _, _, _) => 0x63,
            Log4(_, _, _, _, _) => 0x64,
            Spend(_, _) => 0x65,
            OracleRegister(_, _, _, _, _, _, _) => 0x66,
            OracleQuery(_, _, _, _, _, _, _, _) => 0x67,
            OracleRespond(_, _, _, _, _, _) => 0x68,
            OracleExtend(_, _, _) => 0x69,
            OracleGetAnswer(_, _, _, _, _) => 0x6a,
            OracleGetQuestion(_, _, _, _, _) => 0x6b,
            OracleQueryFee(_, _) => 0x6c,
            AensResolve(_, _, _, _) => 0x6d,
            AensPreclaim(_, _, _) => 0x6e,
            AensClaim(_, _, _, _, _) => 0x6f,
            AensUpdate(_, _, _, _, _, _) => 0x70,
            AensTransfer(_, _, _, _) => 0x71,
            AensRevoke(_, _, _) => 0x72,
            BalanceOther(_, _) => 0x73,
            VerifySig(_, _, _, _) => 0x74,
            VerifySigSecp256K1(_, _, _, _) => 0x75,
            ContractToAddress(_, _) => 0x76,
            AuthTxHash(_) => 0x77,
            OracleCheck(_, _, _, _) => 0x78,
            OracleCheckQuery(_, _, _, _, _) => 0x79,
            IsOracle(_, _) => 0x7a,
            IsContract(_, _) => 0x7b,
            IsPayable(_, _) => 0x7c,
            Creator(_) => 0x7d,
            EcverifySecp256K1(_, _, _, _) => 0x7e,
            EcrecoverSecp256K1(_, _, _) => 0x7f,
            AddressToContract(_, _) => 0x80,
            Bls12_381G1Neg(_, _) => 0x81,
            Bls12_381G1Norm(_, _) => 0x82,
            Bls12_381G1Valid(_, _) => 0x83,
            Bls12_381G1IsZero(_, _) => 0x84,
            Bls12_381G1Add(_, _, _) => 0x85,
            Bls12_381G1Mul(_, _, _) => 0x86,
            Bls12_381G2Neg(_, _) => 0x87,
            Bls12_381G2Norm(_, _) => 0x88,
            Bls12_381G2Valid(_, _) => 0x89,
            Bls12_381G2IsZero(_, _) => 0x8a,
            Bls12_381G2Add(_, _, _) => 0x8b,
            Bls12_381G2Mul(_, _, _) => 0x8c,
            Bls12_381GtInv(_, _) => 0x8d,
            Bls12_381GtAdd(_, _, _) => 0x8e,
            Bls12_381GtMul(_, _, _) => 0x8f,
            Bls12_381GtPow(_, _, _) => 0x90,
            Bls12_381GtIsOne(_, _) => 0x91,
            Bls12_381Pairing(_, _, _) => 0x92,
            Bls12_381MillerLoop(_, _, _) => 0x93,
            Bls12_381FinalExp(_, _) => 0x94,
            Bls12_381IntToFr(_, _) => 0x95,
            Bls12_381IntToFp(_, _) => 0x96,
            Bls12_381FrToInt(_, _) => 0x97,
            Bls12_381FpToInt(_, _) => 0x98,
            AensLookup(_, _) => 0x99,
            OracleExpiry(_, _) => 0x9a,
            AuthTx(_) => 0x9b,
            StrToList(_, _) => 0x9c,
            StrFromList(_, _) => 0x9d,
            StrToUpper(_, _) => 0x9e,
            StrToLower(_, _) => 0x9f,
            CharToInt(_, _) => 0xa0,
            CharFromInt(_, _) => 0xa1,
            CallPgr(_, _, _, _, _, _, _) => 0xa2,
            Create(_, _, _) => 0xa3,
            Clone(_, _, _, _) => 0xa4,
            CloneG(_, _, _, _, _) => 0xa5,
            BytecodeHash(_, _) => 0xa6,
            Fee(_) => 0xa7,
            AddressToBytes(_, _) => 0xa8,
            Poseidon(_, _, _) => 0xa9,
            Mulmod(_, _, _, _) => 0xaa,
            Band(_, _, _) => 0xab,
            Bor(_, _, _) => 0xac,
            Bxor(_, _, _) => 0xad,
            Bnot(_, _) => 0xae,
            Bsl(_, _, _) => 0xaf,
            Bsr(_, _, _) => 0xb0,
            BytesSplitAny(_, _, _) => 0xb1,
            BytesSize(_, _) => 0xb2,
            BytesToFixedSize(_, _, _) => 0xb3,
            IntToBytes(_, _, _) => 0xb4,
            StrToBytes(_, _) => 0xb5,
            DbgLoc(_, _) => 0xb6,
            DbgDef(_, _) => 0xb7,
            DbgUndef(_, _) => 0xb8,
            DbgContract(_) => 0xb9,
            Deactivate => 0xfa,
            Abort(_) => 0xfb,
            Exit(_) => 0xfc,
            Nop => 0xfd,
        }
    }

    pub fn args(&self) -> Vec<crate::code::Arg> {
        use Instruction::*;
        match self {
            Return => vec![],
            Returnr(a1) => vec![a1.clone()],
            Call(a1) => vec![a1.clone()],
            CallR(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            CallT(a1) => vec![a1.clone()],
            CallGr(a1, a2, a3, a4, a5, a6) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone()],
            Jump(a1) => vec![a1.clone()],
            Jumpif(a1, a2) => vec![a1.clone(), a2.clone()],
            SwitchV2(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            SwitchV3(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            SwitchVn(a1, a2) => vec![a1.clone(), a2.clone()],
            CallValue(a1) => vec![a1.clone()],
            Push(a1) => vec![a1.clone()],
            Dupa => vec![],
            Dup(a1) => vec![a1.clone()],
            Pop(a1) => vec![a1.clone()],
            Inca => vec![],
            Inc(a1) => vec![a1.clone()],
            Deca => vec![],
            Dec(a1) => vec![a1.clone()],
            Add(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Sub(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Mul(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Div(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Mod(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Pow(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Store(a1, a2) => vec![a1.clone(), a2.clone()],
            Sha3(a1, a2) => vec![a1.clone(), a2.clone()],
            Sha256(a1, a2) => vec![a1.clone(), a2.clone()],
            Blake2B(a1, a2) => vec![a1.clone(), a2.clone()],
            Lt(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Gt(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Eq(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Elt(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Egt(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Neq(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            And(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Or(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Not(a1, a2) => vec![a1.clone(), a2.clone()],
            Tuple(a1, a2) => vec![a1.clone(), a2.clone()],
            Element(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Setelement(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            MapEmpty(a1) => vec![a1.clone()],
            MapLookup(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            MapLookupd(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            MapUpdate(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            MapDelete(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            MapMember(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            MapFromList(a1, a2) => vec![a1.clone(), a2.clone()],
            MapSize(a1, a2) => vec![a1.clone(), a2.clone()],
            MapToList(a1, a2) => vec![a1.clone(), a2.clone()],
            IsNil(a1, a2) => vec![a1.clone(), a2.clone()],
            Cons(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Hd(a1, a2) => vec![a1.clone(), a2.clone()],
            Tl(a1, a2) => vec![a1.clone(), a2.clone()],
            Length(a1, a2) => vec![a1.clone(), a2.clone()],
            Nil(a1) => vec![a1.clone()],
            Append(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            StrJoin(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            IntToStr(a1, a2) => vec![a1.clone(), a2.clone()],
            AddrToStr(a1, a2) => vec![a1.clone(), a2.clone()],
            StrReverse(a1, a2) => vec![a1.clone(), a2.clone()],
            StrLength(a1, a2) => vec![a1.clone(), a2.clone()],
            BytesToInt(a1, a2) => vec![a1.clone(), a2.clone()],
            BytesToStr(a1, a2) => vec![a1.clone(), a2.clone()],
            BytesConcat(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BytesSplit(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            IntToAddr(a1, a2) => vec![a1.clone(), a2.clone()],
            Variant(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            VariantTest(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            VariantElement(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsNonea => vec![],
            BitsNone(a1) => vec![a1.clone()],
            BitsAlla => vec![],
            BitsAll(a1) => vec![a1.clone()],
            BitsAllN(a1, a2) => vec![a1.clone(), a2.clone()],
            BitsSet(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsClear(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsTest(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsSum(a1, a2) => vec![a1.clone(), a2.clone()],
            BitsOr(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsAnd(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BitsDiff(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Balance(a1) => vec![a1.clone()],
            Origin(a1) => vec![a1.clone()],
            Caller(a1) => vec![a1.clone()],
            Blockhash(a1, a2) => vec![a1.clone(), a2.clone()],
            Beneficiary(a1) => vec![a1.clone()],
            Timestamp(a1) => vec![a1.clone()],
            Generation(a1) => vec![a1.clone()],
            Microblock(a1) => vec![a1.clone()],
            Difficulty(a1) => vec![a1.clone()],
            Gaslimit(a1) => vec![a1.clone()],
            Gas(a1) => vec![a1.clone()],
            Address(a1) => vec![a1.clone()],
            Gasprice(a1) => vec![a1.clone()],
            Log0(a1) => vec![a1.clone()],
            Log1(a1, a2) => vec![a1.clone(), a2.clone()],
            Log2(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Log3(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            Log4(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            Spend(a1, a2) => vec![a1.clone(), a2.clone()],
            OracleRegister(a1, a2, a3, a4, a5, a6, a7) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone(), a7.clone()],
            OracleQuery(a1, a2, a3, a4, a5, a6, a7, a8) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone(), a7.clone(), a8.clone()],
            OracleRespond(a1, a2, a3, a4, a5, a6) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone()],
            OracleExtend(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            OracleGetAnswer(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            OracleGetQuestion(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            OracleQueryFee(a1, a2) => vec![a1.clone(), a2.clone()],
            AensResolve(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            AensPreclaim(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            AensClaim(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            AensUpdate(a1, a2, a3, a4, a5, a6) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone()],
            AensTransfer(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            AensRevoke(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BalanceOther(a1, a2) => vec![a1.clone(), a2.clone()],
            VerifySig(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            VerifySigSecp256K1(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            ContractToAddress(a1, a2) => vec![a1.clone(), a2.clone()],
            AuthTxHash(a1) => vec![a1.clone()],
            OracleCheck(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            OracleCheckQuery(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            IsOracle(a1, a2) => vec![a1.clone(), a2.clone()],
            IsContract(a1, a2) => vec![a1.clone(), a2.clone()],
            IsPayable(a1, a2) => vec![a1.clone(), a2.clone()],
            Creator(a1) => vec![a1.clone()],
            EcverifySecp256K1(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            EcrecoverSecp256K1(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            AddressToContract(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G1Neg(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G1Norm(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G1Valid(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G1IsZero(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G1Add(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381G1Mul(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381G2Neg(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G2Norm(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G2Valid(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G2IsZero(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381G2Add(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381G2Mul(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381GtInv(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381GtAdd(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381GtMul(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381GtPow(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381GtIsOne(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381Pairing(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381MillerLoop(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bls12_381FinalExp(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381IntToFr(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381IntToFp(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381FrToInt(a1, a2) => vec![a1.clone(), a2.clone()],
            Bls12_381FpToInt(a1, a2) => vec![a1.clone(), a2.clone()],
            AensLookup(a1, a2) => vec![a1.clone(), a2.clone()],
            OracleExpiry(a1, a2) => vec![a1.clone(), a2.clone()],
            AuthTx(a1) => vec![a1.clone()],
            StrToList(a1, a2) => vec![a1.clone(), a2.clone()],
            StrFromList(a1, a2) => vec![a1.clone(), a2.clone()],
            StrToUpper(a1, a2) => vec![a1.clone(), a2.clone()],
            StrToLower(a1, a2) => vec![a1.clone(), a2.clone()],
            CharToInt(a1, a2) => vec![a1.clone(), a2.clone()],
            CharFromInt(a1, a2) => vec![a1.clone(), a2.clone()],
            CallPgr(a1, a2, a3, a4, a5, a6, a7) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone(), a6.clone(), a7.clone()],
            Create(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Clone(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            CloneG(a1, a2, a3, a4, a5) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone(), a5.clone()],
            BytecodeHash(a1, a2) => vec![a1.clone(), a2.clone()],
            Fee(a1) => vec![a1.clone()],
            AddressToBytes(a1, a2) => vec![a1.clone(), a2.clone()],
            Poseidon(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Mulmod(a1, a2, a3, a4) => vec![a1.clone(), a2.clone(), a3.clone(), a4.clone()],
            Band(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bor(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bxor(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bnot(a1, a2) => vec![a1.clone(), a2.clone()],
            Bsl(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            Bsr(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BytesSplitAny(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            BytesSize(a1, a2) => vec![a1.clone(), a2.clone()],
            BytesToFixedSize(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            IntToBytes(a1, a2, a3) => vec![a1.clone(), a2.clone(), a3.clone()],
            StrToBytes(a1, a2) => vec![a1.clone(), a2.clone()],
            DbgLoc(a1, a2) => vec![a1.clone(), a2.clone()],
            DbgDef(a1, a2) => vec![a1.clone(), a2.clone()],
            DbgUndef(a1, a2) => vec![a1.clone(), a2.clone()],
            DbgContract(a1) => vec![a1.clone()],
            Deactivate => vec![],
            Abort(a1) => vec![a1.clone()],
            Exit(a1) => vec![a1.clone()],
            Nop => vec![],
        }
    }

    pub fn addressing_mode(&self) -> AddressingMode {
        let args = self.args();
        let padded_args = match args.len() {
            0 => args,
            1..=4 => [[0].repeat(4 - args.len()).iter().map(|_| Arg::Stack(0)).collect(), args].concat(),
            5..=8 => [[0].repeat(8 - args.len()).iter().map(|_| Arg::Stack(0)).collect(), args].concat(),
            _ => panic!("Args length should be less than or equal to 8"),
        };
        if padded_args.len() == 4 {
            let mut m1: u8 = 0;
            for i in 0..4 {
                m1 <<= 2;
                m1 |= modifier_bits(&padded_args[i]);
            }
            AddressingMode::Short(m1)
        } else if padded_args.len() == 8 {
            let mut m1: u8 = 0;
            for i in 0..4 {
                m1 <<= 2;
                m1 |= modifier_bits(&padded_args[i]);
            }
            let mut m2: u8 = 0;
            for i in 4..8 {
                m2 <<= 2;
                m2 |= modifier_bits(&padded_args[i]);
            }
            AddressingMode::Long { low: m1, high: m2 }
        } else {
            panic!("Args were not padded correctly")
        }
    }
}
fn modifier_bits(arg: &crate::code::Arg) -> u8 {
    match arg {
        Arg::Stack(_) => 0b00,
        Arg::Arg(_) => 0b01,
        Arg::Var(_) => 0b10,
        Arg::Immediate(_) => 0b11,
    }
}
