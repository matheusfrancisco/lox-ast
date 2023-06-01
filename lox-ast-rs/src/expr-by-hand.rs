enum Expr {
  Binary(BinaryExpr),
  Grouping(GroupingExpr),
  Literal(LiteralExpr),
  Unary(UnaryExpr),
}

struct BinaryExpr {
  left: Box<Expr>,
  operator: Token,
  right: Box<Expr>,
}

struct GroupingExpr {
  expression: Box<Expr>,
}

struct LiteralExpr {
  value: Literal,
}

struct UnaryExpr {
  operator: Token,
  right: Box<Expr>,
}

pub trait ExprVisitor<T> {
  fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
  fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
  fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
  fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
}

impl BinaryExpr {
  fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
    visitor.visit_binary_expr(self)
  }
}

impl GroupingExpr {
  fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
    visitor.visit_grouping_expr(self)
  }
}

impl LiteralExpr {
  fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
    visitor.visit_literal_expr(self)
  }
}

impl UnaryExpr {
  fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
    visitor.visit_unary_expr(self)
  }
}


