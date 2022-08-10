

abstract Class Expr {
    interface Visitor<R>  {
	R visistBinaryExpr(Binary expr);
    }

    static class Binary extends Expr {
	Binary(Expr left, Token operator, Expr right) {
	    this.left = left;
	    this.operator = operator;
	    this.right = right;
	}

	@Override
	<R> R accept(Visitor<R> visitor) {
	    return visitor.visitBinaryExpr(this);
	}

	final Expr left;
	final Token operator;
	final Expr right;
    }
}
