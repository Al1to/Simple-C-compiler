int main () {
    float d = 123.0 + 456.0 + 789.6 + 9.0 + 7.9 + 4.0 + 1.0;
}

void test(int arg1, float arg2) {}

// tokens:
// [IntT, Id { tok_lexeme: "main" }, LP, RP, LC, FloatT, Id { tok_lexeme: "d" }, Assignment, FloatN { num: 123.0 }, Plus, FloatN { num: 456.0 }, Plus, FloatN { num: 789.6 }, Plus, FloatN { num: 9.0 }, Plus, FloatN { num: 7.9 }, Plus, FloatN { num: 4.0 }, Plus, FloatN { num: 1.0 }, Semicolon, RC, VoidT, Id { tok_lexeme: "test" }, LP, IntT, Id { tok_lexeme: "arg1" }, Comma, FloatT, Id { tok_lexeme: "arg2" }, RP, LC, RC]

// ast:
// [Block { vec: [RefCell { value: FuncDef { ret_t: IntT, name: Id { id: "main" }, args: Block { vec: [] }, block: Block { vec: [RefCell { value: Expr { e: BinE { exprl: Var { var_t: FloatT, name: Id { id: "d" } }, op: Assignment, exprr: Expr { e: BinE { exprl: FloatN { n: 123.0 }, op: Add, exprr: Expr { e: BinE { exprl: FloatN { n: 456.0 }, op: Add, exprr: Expr { e: BinE { exprl: FloatN { n: 789.6 }, op: Add, exprr: Expr { e: BinE { exprl: FloatN { n: 9.0 }, op: Add, exprr: Expr { e: BinE { exprl: FloatN { n: 7.9 }, op: Add, exprr: Expr { e: BinE { exprl: FloatN { n: 4.0 }, op: Add, exprr: Expr { e: FloatN { n: 1.0 } } } } } } } } } } } } } } } } }] } } }, RefCell { value: FuncDef { ret_t: VoidT, name: Id { id: "test" }, args: Block { vec: [RefCell { value: Var { var_t: IntT, name: Id { id: "arg1" } } }, RefCell { value: Var { var_t: FloatT, name: Id { id: "arg2" } } }] }, block: Block { vec: [] } } }] }]