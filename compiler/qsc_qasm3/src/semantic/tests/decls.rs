// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use expect_test::expect;

use super::check;

#[test]
fn scalar_types_without_init_exprs_have_default_values() {
    check(
        r#"
        bit a;
        bool c;
        complex d;
        complex[float] e;
        complex[float[32]] f;
        creg g;
        float j;
        float[5] k;
        int l;
        int[5] m;
        uint o;
        uint[5] p;
        "#,
        &expect![[r#"

                Stmt [9-15]
                    StmtKind: ClassicalDeclarationStmt [9-15]: 6, ValueExpression Expr [0-0]: Lit: Int(0) -> Bit(false)
                Stmt [24-31]
                    StmtKind: ClassicalDeclarationStmt [24-31]: 7, ValueExpression Expr [0-0]: Lit: Bool(false) -> Bool(false)
                Stmt [40-50]
                    StmtKind: ClassicalDeclarationStmt [40-50]: 8, ValueExpression Expr [0-0]: Lit: Complex(0.0, 0.0) -> Complex(None, false)
                Stmt [59-76]
                    StmtKind: ClassicalDeclarationStmt [59-76]: 9, ValueExpression Expr [0-0]: Lit: Complex(0.0, 0.0) -> Complex(None, false)
                Stmt [85-106]
                    StmtKind: ClassicalDeclarationStmt [85-106]: 10, ValueExpression Expr [0-0]: Lit: Complex(0.0, 0.0) -> Complex(Some(32), false)
                Stmt [115-122]
                    StmtKind: ClassicalDeclarationStmt [115-122]: 11, ValueExpression Expr [0-0]: Lit: Int(0) -> Bit(false)
                Stmt [131-139]
                    StmtKind: ClassicalDeclarationStmt [131-139]: 12, ValueExpression Expr [0-0]: Lit: Float(0.0) -> Float(None, false)
                Stmt [148-159]
                    StmtKind: ClassicalDeclarationStmt [148-159]: 13, ValueExpression Expr [0-0]: Lit: Float(0.0) -> Float(Some(5), false)
                Stmt [168-174]
                    StmtKind: ClassicalDeclarationStmt [168-174]: 14, ValueExpression Expr [0-0]: Lit: Int(0) -> Int(None, false)
                Stmt [183-192]
                    StmtKind: ClassicalDeclarationStmt [183-192]: 15, ValueExpression Expr [0-0]: Lit: Int(0) -> Int(Some(5), false)
                Stmt [201-208]
                    StmtKind: ClassicalDeclarationStmt [201-208]: 16, ValueExpression Expr [0-0]: Lit: Int(0) -> UInt(None, false)
                Stmt [217-227]
                    StmtKind: ClassicalDeclarationStmt [217-227]: 17, ValueExpression Expr [0-0]: Lit: Int(0) -> UInt(Some(5), false)"#]],
    );
}

#[test]
#[ignore = "Not yet implemented"]
fn special_case_scalar_type_arrays_without_init_exprs() {
    check(
        r#"
        bit[5] a;
        creg b[5];
        "#,
        &expect![[r#"


            [Qsc.Qasm3.Compile.Unimplemented

              x this statement is not yet handled during OpenQASM 3 import: bit array
              | default value
               ,-[test:2:9]
             1 |
             2 |         bit[5] a;
               :         ^^^^^^^^^
             3 |         creg b[5];
               `----
            , Qsc.Qasm3.Compile.Unimplemented

              x this statement is not yet handled during OpenQASM 3 import: bit array
              | default value
               ,-[test:3:9]
             2 |         bit[5] a;
             3 |         creg b[5];
               :         ^^^^^^^^^^
             4 |
               `----
            ]"#]],
    );
}

#[test]
#[ignore = "Not yet implemented"]
fn duration_and_stretch_types_without_init_exprs() {
    check(
        r#"
        duration i;
        stretch n;
        "#,
        &expect![[r#"


            [Qsc.Qasm3.Compile.NotSupported

              x Duration type values are not supported.
               ,-[test:2:9]
             1 |
             2 |         duration i;
               :         ^^^^^^^^
             3 |         stretch n;
               `----
            , Qsc.Qasm3.Compile.NotSupported

              x Stretch type values are not supported.
               ,-[test:3:9]
             2 |         duration i;
             3 |         stretch n;
               :         ^^^^^^^
             4 |
               `----
            ]"#]],
    );
}

#[test]
fn scalar_ty_designator_must_be_positive() {
    check(
        "int[-5] i;",
        &expect![[r#"


            [Qsc.Qasm3.Compile.DesignatorMustBePositiveIntLiteral

              x Designator must be a positive literal integer.
               ,-[test:1:5]
             1 | int[-5] i;
               :     ^^
               `----
            ]"#]],
    );
}

#[test]
fn scalar_ty_designator_must_be_int_literal() {
    check(
        r#"int[size] i; float[0.0] j;"#,
        &expect![[r#"


            [Qsc.Qasm3.Compile.DesignatorMustBePositiveIntLiteral

              x Designator must be a positive literal integer.
               ,-[test:1:5]
             1 | int[size] i; float[0.0] j;
               :     ^^^^
               `----
            , Qsc.Qasm3.Compile.DesignatorMustBePositiveIntLiteral

              x Designator must be a positive literal integer.
               ,-[test:1:20]
             1 | int[size] i; float[0.0] j;
               :                    ^^^
               `----
            ]"#]],
    );
}

#[test]
fn bit_decl_with_lit_init_exprs() {
    check(
        r#"
        bit a = 1;
        bit b = 0;
        "#,
        &expect![[r#"

                Stmt [9-19]
                    StmtKind: ClassicalDeclarationStmt [9-19]: 6, ValueExpression Expr [17-18]: Lit: Int(1) -> Bit(false)
                Stmt [28-38]
                    StmtKind: ClassicalDeclarationStmt [28-38]: 7, ValueExpression Expr [36-37]: Lit: Int(0) -> Bit(false)"#]],
    );
}

#[test]
fn const_bit_decl_with_lit_init_exprs() {
    check(
        r#"
        const bit a = 1;
        const bit b = 0;
        "#,
        &expect![[r#"

                Stmt [9-25]
                    StmtKind: ClassicalDeclarationStmt [9-25]: 6, ValueExpression Expr [23-24]: Lit: Int(1) -> Bit(true)
                Stmt [34-50]
                    StmtKind: ClassicalDeclarationStmt [34-50]: 7, ValueExpression Expr [48-49]: Lit: Int(0) -> Bit(true)"#]],
    );
}

#[test]
fn bool_decl_with_lit_init_exprs() {
    check(
        r#"
        bool a = true;
        bool b = false;
        bool c = !true;
        bool d = !false;
        bool e = !!true;
        bool f = !!false;
        "#,
        &expect![[r#"

                Stmt [9-23]
                    StmtKind: ClassicalDeclarationStmt [9-23]: 6, ValueExpression Expr [18-22]: [Kind: Lit: Bool(true), eTy: Bool(true)]
                Stmt [32-47]
                    StmtKind: ClassicalDeclarationStmt [32-47]: 7, ValueExpression Expr [41-46]: [Kind: Lit: Bool(false), eTy: Bool(true)]
                Stmt [56-71]
                    StmtKind: ClassicalDeclarationStmt [56-71]: 8, ValueExpression Expr [66-70]: [Kind: Lit: Bool(false), eTy: Bool(true)]
                Stmt [80-96]
                    StmtKind: ClassicalDeclarationStmt [80-96]: 9, ValueExpression Expr [90-95]: [Kind: Lit: Bool(true), eTy: Bool(true)]
                Stmt [105-121]
                    StmtKind: ClassicalDeclarationStmt [105-121]: 10, ValueExpression Expr [116-120]: [Kind: UnOp (NotL):
                        Expr [116-120]: [Kind: Lit: Bool(false), eTy: Bool(true)], eTy: Bool(true)]
                Stmt [130-147]
                    StmtKind: ClassicalDeclarationStmt [130-147]: 11, ValueExpression Expr [141-146]: [Kind: UnOp (NotL):
                        Expr [141-146]: [Kind: Lit: Bool(true), eTy: Bool(true)], eTy: Bool(true)]"#]],
    );
}

#[test]
fn const_bool_decl_with_lit_init_exprs() {
    check(
        r#"
        const bool a = true;
        const bool b = false;
        const bool c = !true;
        const bool d = !false;
        const bool e = !!true;
        const bool f = !!false;
        "#,
        &expect![[r#"

                Stmt [9-29]
                    StmtKind: ClassicalDeclarationStmt [9-29]: 6, ValueExpression Expr [24-28]: Lit: Bool(true) -> Bool(true)
                Stmt [38-59]
                    StmtKind: ClassicalDeclarationStmt [38-59]: 7, ValueExpression Expr [53-58]: Lit: Bool(false) -> Bool(true)
                Stmt [68-89]
                    StmtKind: ClassicalDeclarationStmt [68-89]: 8, ValueExpression Expr [84-88]: Lit: Bool(false) -> Bool(true)
                Stmt [98-120]
                    StmtKind: ClassicalDeclarationStmt [98-120]: 9, ValueExpression Expr [114-119]: Lit: Bool(true) -> Bool(true)
                Stmt [129-151]
                    StmtKind: ClassicalDeclarationStmt [129-151]: 10, ValueExpression Expr [146-150]: UnOp (NotL):
                        Expr [146-150]: Lit: Bool(false) -> Bool(true) -> Bool(true)
                Stmt [160-183]
                    StmtKind: ClassicalDeclarationStmt [160-183]: 11, ValueExpression Expr [177-182]: UnOp (NotL):
                        Expr [177-182]: Lit: Bool(true) -> Bool(true) -> Bool(true)"#]],
    );
}

#[test]
fn complex_decl_with_lit_init_exprs() {
    check(
        r#"
        complex a = 0.1im;
        complex aneg = -0.1im;
        complex b = 0.2;
        complex bneg = -0.2;
        complex c = 1;
        complex cneg = -1;
        complex[float] d = 0.04;
        complex[float] dneg = -0.04;
        complex[float] e = 0.02im;
        complex[float] eneg = -0.02im;
        complex[float[32]] f = 0.03im;
        complex[float[32]] fneg = -0.03im;
        "#,
        &expect![[r#"

                Stmt [9-27]
                    StmtKind: ClassicalDeclarationStmt [9-27]: 6, ValueExpression Expr [21-26]: Lit: Complex(0.0, 0.1) -> Complex(None, true)
                Stmt [36-58]
                    StmtKind: ClassicalDeclarationStmt [36-58]: 7, ValueExpression Expr [52-57]: Lit: Complex(0.0, -0.1) -> Complex(None, true)
                Stmt [67-83]
                    StmtKind: ClassicalDeclarationStmt [67-83]: 8, ValueExpression Expr [79-82]: Lit: Complex(0.2, 0.0) -> Complex(None, false)
                Stmt [92-112]
                    StmtKind: ClassicalDeclarationStmt [92-112]: 9, ValueExpression Expr [108-111]: Lit: Complex(-0.2, 0.0) -> Complex(None, false)
                Stmt [121-135]
                    StmtKind: ClassicalDeclarationStmt [121-135]: 10, ValueExpression Expr [133-134]: Lit: Complex(1.0, 0.0) -> Complex(None, false)
                Stmt [144-162]
                    StmtKind: ClassicalDeclarationStmt [144-162]: 11, ValueExpression Expr [160-161]: Lit: Complex(-1.0, 0.0) -> Complex(None, false)
                Stmt [171-195]
                    StmtKind: ClassicalDeclarationStmt [171-195]: 12, ValueExpression Expr [190-194]: Lit: Complex(0.04, 0.0) -> Complex(None, false)
                Stmt [204-232]
                    StmtKind: ClassicalDeclarationStmt [204-232]: 13, ValueExpression Expr [227-231]: Lit: Complex(-0.04, 0.0) -> Complex(None, false)
                Stmt [241-267]
                    StmtKind: ClassicalDeclarationStmt [241-267]: 14, ValueExpression Expr [260-266]: Lit: Complex(0.0, 0.02) -> Complex(None, true)
                Stmt [276-306]
                    StmtKind: ClassicalDeclarationStmt [276-306]: 15, ValueExpression Expr [299-305]: Lit: Complex(0.0, -0.02) -> Complex(None, true)
                Stmt [315-345]
                    StmtKind: ClassicalDeclarationStmt [315-345]: 16, ValueExpression Expr [338-344]: Lit: Complex(0.0, 0.03) -> Complex(Some(32), false)
                Stmt [354-388]
                    StmtKind: ClassicalDeclarationStmt [354-388]: 17, ValueExpression Expr [381-387]: Lit: Complex(0.0, -0.03) -> Complex(Some(32), false)"#]],
    );
}

#[test]
fn const_complex_decl_with_lit_init_exprs() {
    check(
        r#"
        const complex a = 0.1im;
        const complex aneg = -0.1im;
        const complex b = 0.2;
        const complex bneg = -0.2;
        const complex c = 1;
        const complex cneg = -1;
        const complex[float] d = 0.04;
        const complex[float] dneg = -0.04;
        const complex[float] e = 0.02im;
        const complex[float] eneg = -0.02im;
        const complex[float[32]] f = 0.03im;
        const complex[float[32]] fneg = -0.03im;
        "#,
        &expect![[r#"

                Stmt [9-33]
                    StmtKind: ClassicalDeclarationStmt [9-33]: 6, ValueExpression Expr [27-32]: Lit: Complex(0.0, 0.1) -> Complex(None, true)
                Stmt [42-70]
                    StmtKind: ClassicalDeclarationStmt [42-70]: 7, ValueExpression Expr [64-69]: Lit: Complex(0.0, -0.1) -> Complex(None, true)
                Stmt [79-101]
                    StmtKind: ClassicalDeclarationStmt [79-101]: 8, ValueExpression Expr [97-100]: Lit: Complex(0.2, 0.0) -> Complex(None, true)
                Stmt [110-136]
                    StmtKind: ClassicalDeclarationStmt [110-136]: 9, ValueExpression Expr [132-135]: Lit: Complex(-0.2, 0.0) -> Complex(None, true)
                Stmt [145-165]
                    StmtKind: ClassicalDeclarationStmt [145-165]: 10, ValueExpression Expr [163-164]: Lit: Complex(1.0, 0.0) -> Complex(None, true)
                Stmt [174-198]
                    StmtKind: ClassicalDeclarationStmt [174-198]: 11, ValueExpression Expr [196-197]: Lit: Complex(-1.0, 0.0) -> Complex(None, true)
                Stmt [207-237]
                    StmtKind: ClassicalDeclarationStmt [207-237]: 12, ValueExpression Expr [232-236]: Lit: Complex(0.04, 0.0) -> Complex(None, true)
                Stmt [246-280]
                    StmtKind: ClassicalDeclarationStmt [246-280]: 13, ValueExpression Expr [275-279]: Lit: Complex(-0.04, 0.0) -> Complex(None, true)
                Stmt [289-321]
                    StmtKind: ClassicalDeclarationStmt [289-321]: 14, ValueExpression Expr [314-320]: Lit: Complex(0.0, 0.02) -> Complex(None, true)
                Stmt [330-366]
                    StmtKind: ClassicalDeclarationStmt [330-366]: 15, ValueExpression Expr [359-365]: Lit: Complex(0.0, -0.02) -> Complex(None, true)
                Stmt [375-411]
                    StmtKind: ClassicalDeclarationStmt [375-411]: 16, ValueExpression Expr [404-410]: Lit: Complex(0.0, 0.03) -> Complex(Some(32), true)
                Stmt [420-460]
                    StmtKind: ClassicalDeclarationStmt [420-460]: 17, ValueExpression Expr [453-459]: Lit: Complex(0.0, -0.03) -> Complex(Some(32), true)"#]],
    );
}

#[test]
fn float_decl_with_lit_init_exprs() {
    check(
        r#"
        float a = 0.04;
        float aneg = -0.04;
        float bnegneg = --0.04;
        float[5] c = 0.05;
        float[5] cneg = -0.05;
        float[5] cnegneg = --0.05;
        "#,
        &expect![[r#"

                Stmt [9-24]
                    StmtKind: ClassicalDeclarationStmt [9-24]: 6, ValueExpression Expr [19-23]: Lit: Float(0.04) -> Float(None, true)
                Stmt [33-52]
                    StmtKind: ClassicalDeclarationStmt [33-52]: 7, ValueExpression Expr [47-51]: Lit: Float(-0.04) -> Float(None, true)
                Stmt [61-84]
                    StmtKind: ClassicalDeclarationStmt [61-84]: 8, ValueExpression Expr [79-83]: UnOp (Neg):
                        Expr [79-83]: Lit: Float(-0.04) -> Float(None, true) -> Float(None, true)
                Stmt [93-111]
                    StmtKind: ClassicalDeclarationStmt [93-111]: 9, ValueExpression Expr [106-110]: Lit: Float(0.05) -> Float(Some(5), false)
                Stmt [120-142]
                    StmtKind: ClassicalDeclarationStmt [120-142]: 10, ValueExpression Expr [137-141]: Lit: Float(-0.05) -> Float(Some(5), false)
                Stmt [151-177]
                    StmtKind: ClassicalDeclarationStmt [151-177]: 11, ValueExpression Expr [172-176]: UnOp (Neg):
                        Expr [172-176]: Lit: Float(-0.05) -> Float(None, true) -> Float(Some(5), false)"#]],
    );
}

#[test]
fn const_float_decl_with_lit_init_exprs() {
    check(
        r#"
        const float a = 0.04;
        const float aneg = -0.04;
        const float bnegneg = --0.04;
        const float[5] c = 0.05;
        const float[5] cneg = -0.05;
        const float[5] cnegneg = --0.05;
        "#,
        &expect![[r#"

                Stmt [9-30]
                    StmtKind: ClassicalDeclarationStmt [9-30]: 6, ValueExpression Expr [25-29]: Lit: Float(0.04) -> Float(None, true)
                Stmt [39-64]
                    StmtKind: ClassicalDeclarationStmt [39-64]: 7, ValueExpression Expr [59-63]: Lit: Float(-0.04) -> Float(None, true)
                Stmt [73-102]
                    StmtKind: ClassicalDeclarationStmt [73-102]: 8, ValueExpression Expr [97-101]: UnOp (Neg):
                        Expr [97-101]: Lit: Float(-0.04) -> Float(None, true) -> Float(None, true)
                Stmt [111-135]
                    StmtKind: ClassicalDeclarationStmt [111-135]: 9, ValueExpression Expr [130-134]: Lit: Float(0.05) -> Float(Some(5), true)
                Stmt [144-172]
                    StmtKind: ClassicalDeclarationStmt [144-172]: 10, ValueExpression Expr [167-171]: Lit: Float(-0.05) -> Float(Some(5), true)
                Stmt [181-213]
                    StmtKind: ClassicalDeclarationStmt [181-213]: 11, ValueExpression Expr [208-212]: UnOp (Neg):
                        Expr [208-212]: Lit: Float(-0.05) -> Float(None, true) -> Float(Some(5), true)"#]],
    );
}

#[test]
fn int_decl_with_lit_init_exprs() {
    check(
        r#"
        int a = 2;
        int aneg = -2;
        int anegneg = --2;
        int[5] b = 3;
        int[5] bneg = -3;
        int[5] bnegneg = --3;
        "#,
        &expect![[r#"

                Stmt [9-19]
                    StmtKind: ClassicalDeclarationStmt [9-19]: 6, ValueExpression Expr [17-18]: [Kind: Lit: Int(2), eTy: Int(None, true)]
                Stmt [28-42]
                    StmtKind: ClassicalDeclarationStmt [28-42]: 7, ValueExpression Expr [40-41]: [Kind: Lit: Int(-2), eTy: Int(None, true)]
                Stmt [51-69]
                    StmtKind: ClassicalDeclarationStmt [51-69]: 8, ValueExpression Expr [67-68]: [Kind: UnOp (Neg):
                        Expr [67-68]: [Kind: Lit: Int(-2), eTy: Int(None, true)], eTy: Int(None, true)]
                Stmt [78-91]
                    StmtKind: ClassicalDeclarationStmt [78-91]: 9, ValueExpression Expr [89-90]: [Kind: Lit: Int(3), eTy: Int(Some(5), false)]
                Stmt [100-117]
                    StmtKind: ClassicalDeclarationStmt [100-117]: 10, ValueExpression Expr [115-116]: [Kind: Lit: Int(-3), eTy: Int(Some(5), false)]
                Stmt [126-147]
                    StmtKind: ClassicalDeclarationStmt [126-147]: 11, ValueExpression Expr [0-0]: [Kind: Cast [0-0]:
                        Target Ty: Int(Some(5), false)
                        Source Expr: Expr [145-146]: [Kind: UnOp (Neg):
                            Expr [145-146]: [Kind: Lit: Int(-3), eTy: Int(None, true)], eTy: Int(None, true)], eTy: Int(Some(5), false)]"#]],
    );
}

#[test]
fn const_int_decl_with_lit_init_exprs() {
    check(
        r#"
        const int a = 2;
        const int aneg = -2;
        const int anegneg = --2;
        const int[5] b = 3;
        const int[5] bneg = -3;
        const int[5] bnegneg = --3;
        "#,
        &expect![[r#"

                Stmt [9-25]
                    StmtKind: ClassicalDeclarationStmt [9-25]: 6, ValueExpression Expr [23-24]: Lit: Int(2) -> Int(None, true)
                Stmt [34-54]
                    StmtKind: ClassicalDeclarationStmt [34-54]: 7, ValueExpression Expr [52-53]: Lit: Int(-2) -> Int(None, true)
                Stmt [63-87]
                    StmtKind: ClassicalDeclarationStmt [63-87]: 8, ValueExpression Expr [85-86]: UnOp (Neg):
                        Expr [85-86]: Lit: Int(-2) -> Int(None, true) -> Int(None, true)
                Stmt [96-115]
                    StmtKind: ClassicalDeclarationStmt [96-115]: 9, ValueExpression Expr [113-114]: Lit: Int(3) -> Int(Some(5), true)
                Stmt [124-147]
                    StmtKind: ClassicalDeclarationStmt [124-147]: 10, ValueExpression Expr [145-146]: Lit: Int(-3) -> Int(Some(5), true)
                Stmt [156-183]
                    StmtKind: ClassicalDeclarationStmt [156-183]: 11, ValueExpression Expr [0-0]: Cast [0-0]:
                        Int(Some(5), true)
                        Expr [181-182]: UnOp (Neg):
                            Expr [181-182]: Lit: Int(-3) -> Int(None, true) -> Int(None, true) -> Int(Some(5), true)"#]],
    );
}

#[test]
fn uint_decl_with_lit_init_exprs() {
    check(
        r#"
        uint a = 4;
        uint aneg = -4; // error
        uint anegneg = --4; // error
        uint[5] b = 5;
        uint[5] bneg = -5; // error
        uint[5] bnegneg = --5; // error
        "#,
        &expect![[r#"

                Stmt [9-20]
                    StmtKind: ClassicalDeclarationStmt [9-20]: 6, ValueExpression Expr [18-19]: Lit: Int(4) -> UInt(None, false)
                Stmt [62-81]
                    StmtKind: ClassicalDeclarationStmt [62-81]: 8, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(None, false)
                        Expr [79-80]: UnOp (Neg):
                            Expr [79-80]: Lit: Int(-4) -> Int(None, true) -> Int(None, true) -> UInt(None, false)
                Stmt [99-113]
                    StmtKind: ClassicalDeclarationStmt [99-113]: 9, ValueExpression Expr [111-112]: Lit: Int(5) -> UInt(Some(5), false)
                Stmt [158-180]
                    StmtKind: ClassicalDeclarationStmt [158-180]: 11, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(Some(5), false)
                        Expr [178-179]: UnOp (Neg):
                            Expr [178-179]: Lit: Int(-5) -> Int(None, true) -> Int(None, true) -> UInt(Some(5), false)

            [Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(None, false) type.
               ,-[test:3:9]
             2 |         uint a = 4;
             3 |         uint aneg = -4; // error
               :         ^^^^^^^^^^^^^^^
             4 |         uint anegneg = --4; // error
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(Some(5), false) type.
               ,-[test:6:9]
             5 |         uint[5] b = 5;
             6 |         uint[5] bneg = -5; // error
               :         ^^^^^^^^^^^^^^^^^^
             7 |         uint[5] bnegneg = --5; // error
               `----
            ]"#]],
    );
}

#[test]
fn const_uint_decl_with_lit_init_exprs() {
    check(
        r#"
        const uint a = 4;
        const uint aneg = -4; // error
        const uint anegneg = --4; // error
        const uint[5] b = 5;
        const uint[5] bneg = -5; // error
        const uint[5] bnegneg = --5; // error
        "#,
        &expect![[r#"

                Stmt [9-26]
                    StmtKind: ClassicalDeclarationStmt [9-26]: 6, ValueExpression Expr [24-25]: Lit: Int(4) -> UInt(None, true)
                Stmt [74-99]
                    StmtKind: ClassicalDeclarationStmt [74-99]: 8, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(None, true)
                        Expr [97-98]: UnOp (Neg):
                            Expr [97-98]: Lit: Int(-4) -> Int(None, true) -> Int(None, true) -> UInt(None, true)
                Stmt [117-137]
                    StmtKind: ClassicalDeclarationStmt [117-137]: 9, ValueExpression Expr [135-136]: Lit: Int(5) -> UInt(Some(5), true)
                Stmt [188-216]
                    StmtKind: ClassicalDeclarationStmt [188-216]: 11, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(Some(5), true)
                        Expr [214-215]: UnOp (Neg):
                            Expr [214-215]: Lit: Int(-5) -> Int(None, true) -> Int(None, true) -> UInt(Some(5), true)

            [Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(None, true) type.
               ,-[test:3:9]
             2 |         const uint a = 4;
             3 |         const uint aneg = -4; // error
               :         ^^^^^^^^^^^^^^^^^^^^^
             4 |         const uint anegneg = --4; // error
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(Some(5), true) type.
               ,-[test:6:9]
             5 |         const uint[5] b = 5;
             6 |         const uint[5] bneg = -5; // error
               :         ^^^^^^^^^^^^^^^^^^^^^^^^
             7 |         const uint[5] bnegneg = --5; // error
               `----
            ]"#]],
    );
}

#[test]
fn bigint_decl_with_lit_init_exprs() {
    check(
        r#"
        int a = 19_223_372_036_854_775_807;
        int aneg = -19_223_372_036_854_775_807;
        int anegneg = --19_223_372_036_854_775_807;
        uint b = 19_223_372_036_854_775_807;
        uint bneg = -19_223_372_036_854_775_807;
        uint bnegneg = --19_223_372_036_854_775_807;
        "#,
        &expect![[r#"

                Stmt [53-92]
                    StmtKind: ClassicalDeclarationStmt [53-92]: 7, ValueExpression Expr [65-91]: Lit: BigInt(-19223372036854775807) -> Int(None, true)
                Stmt [101-144]
                    StmtKind: ClassicalDeclarationStmt [101-144]: 8, ValueExpression Expr [117-143]: UnOp (Neg):
                        Expr [117-143]: Lit: BigInt(-19223372036854775807) -> Int(None, true) -> Int(None, true)
                Stmt [247-291]
                    StmtKind: ClassicalDeclarationStmt [247-291]: 11, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(None, false)
                        Expr [264-290]: UnOp (Neg):
                            Expr [264-290]: Lit: BigInt(-19223372036854775807) -> Int(None, true) -> Int(None, true) -> UInt(None, false)

            [Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Err type to a classical variable of Int(None,
              | false) type.
               ,-[test:2:9]
             1 |
             2 |         int a = 19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             3 |         int aneg = -19_223_372_036_854_775_807;
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Err type to a classical variable of UInt(None,
              | false) type.
               ,-[test:5:9]
             4 |         int anegneg = --19_223_372_036_854_775_807;
             5 |         uint b = 19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             6 |         uint bneg = -19_223_372_036_854_775_807;
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(None, false) type.
               ,-[test:6:9]
             5 |         uint b = 19_223_372_036_854_775_807;
             6 |         uint bneg = -19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             7 |         uint bnegneg = --19_223_372_036_854_775_807;
               `----
            ]"#]],
    );
}

#[test]
fn const_bigint_decl_with_lit_init_exprs() {
    check(
        r#"
        const int a = 19_223_372_036_854_775_807;
        const int aneg = -19_223_372_036_854_775_807;
        const int anegneg = --19_223_372_036_854_775_807;
        const uint b = 19_223_372_036_854_775_807;
        const uint bneg = -19_223_372_036_854_775_807;
        const uint bnegneg = --19_223_372_036_854_775_807;
        "#,
        &expect![[r#"

                Stmt [59-104]
                    StmtKind: ClassicalDeclarationStmt [59-104]: 7, ValueExpression Expr [77-103]: Lit: BigInt(-19223372036854775807) -> Int(None, true)
                Stmt [113-162]
                    StmtKind: ClassicalDeclarationStmt [113-162]: 8, ValueExpression Expr [135-161]: UnOp (Neg):
                        Expr [135-161]: Lit: BigInt(-19223372036854775807) -> Int(None, true) -> Int(None, true)
                Stmt [277-327]
                    StmtKind: ClassicalDeclarationStmt [277-327]: 11, ValueExpression Expr [0-0]: Cast [0-0]:
                        UInt(None, true)
                        Expr [300-326]: UnOp (Neg):
                            Expr [300-326]: Lit: BigInt(-19223372036854775807) -> Int(None, true) -> Int(None, true) -> UInt(None, true)

            [Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Err type to a classical variable of Int(None,
              | true) type.
               ,-[test:2:9]
             1 |
             2 |         const int a = 19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             3 |         const int aneg = -19_223_372_036_854_775_807;
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Err type to a classical variable of UInt(None,
              | true) type.
               ,-[test:5:9]
             4 |         const int anegneg = --19_223_372_036_854_775_807;
             5 |         const uint b = 19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             6 |         const uint bneg = -19_223_372_036_854_775_807;
               `----
            , Qsc.Qasm3.Compile.CannotAssignToType

              x Cannot assign a value of Int(None, true) type to a classical variable of
              | UInt(None, true) type.
               ,-[test:6:9]
             5 |         const uint b = 19_223_372_036_854_775_807;
             6 |         const uint bneg = -19_223_372_036_854_775_807;
               :         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
             7 |         const uint bnegneg = --19_223_372_036_854_775_807;
               `----
            ]"#]],
    );
}
