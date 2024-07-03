// customJSXIdentifierPlugin.js
module.exports = function customJSXIdentifierPlugin({ types: t }) {
  return {
    visitor: {
      FunctionDeclaration(path) {
        if (containsJSXReturn(path.node.body.body, t)) {
          addJSXReturnFlag(path, path.node.id.name, t);
        }
      },
      FunctionExpression(path) {
        if (containsJSXReturn(path.node.body.body, t)) {
          handleFunctionExpressionOrArrow(path, t);
        }
      },
      ArrowFunctionExpression(path) {
        if (t.isJSXElement(path.node.body) || (t.isBlockStatement(path.node.body) && containsJSXReturn(path.node.body.body, t))) {
          handleFunctionExpressionOrArrow(path, t);
        }
      },
    }
  };
};

function containsJSXReturn(body, t) {
  return body.some(statement => t.isReturnStatement(statement) && t.isJSXElement(statement.argument));
}

function addJSXReturnFlag(path, functionName, t) {
  path.insertAfter(
    t.expressionStatement(
      t.assignmentExpression(
        '=',
        t.memberExpression(t.identifier(functionName), t.identifier('__returnsJSX')),
        t.booleanLiteral(true)
      )
    )
  );
}

function handleFunctionExpressionOrArrow(path, t) {
  // Check for variable declaration
  if (t.isVariableDeclarator(path.parent) && t.isIdentifier(path.parent.id)) {
    const functionName = path.parent.id.name;
    path.findParent(p => p.isVariableDeclaration()).insertAfter(
      t.expressionStatement(
        t.assignmentExpression(
          '=',
          t.memberExpression(t.identifier(functionName), t.identifier('__returnsJSX')),
          t.booleanLiteral(true)
        )
      )
    );
  }
  // Check for variable assignment
  else if (t.isAssignmentExpression(path.parent) && t.isIdentifier(path.parent.left)) {
    const functionName = path.parent.left.name;
    path.parentPath.insertAfter(
      t.expressionStatement(
        t.assignmentExpression(
          '=',
          t.memberExpression(t.identifier(functionName), t.identifier('__returnsJSX')),
          t.booleanLiteral(true)
        )
      )
    );
  }
}
