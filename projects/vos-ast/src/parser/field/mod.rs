use super::*;

impl FieldStatementNode {
    pub fn as_field(&self) -> VosResult<FieldStatement> {
        let mut field = FieldStatement::default();
        field.name = self.key.as_identifier();
        field.typing = self.r#type.as_field_type()?;
        field.value = as_value(&self.value)?;
        Ok(field)
    }
}

impl ConstraintStatementNode {
    pub fn as_constraint(&self) -> VosResult<ConstraintStatement> {
        Ok(ConstraintStatement { name: self.key.as_identifier(), value: as_value(&self.value)? })
    }
}

impl ValueNode {
    pub fn as_value(&self) -> VosResult<ValueStatement> {
        match self {
            ValueNode::DefaultNode(v) => Ok(v.as_value()),
            ValueNode::BooleanNode(v) => Ok(v.as_value()),
            ValueNode::NumNode(v) => v.as_value(),
            ValueNode::NamespaceNode(v) => Ok(v.as_value()),
        }
    }
}

impl DefaultNode {
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Default, range: as_range(&self.position) }
    }
}

impl BooleanNode {
    pub fn as_bool(&self) -> bool {
        match self.string.as_str() {
            "true" => true,
            _ => false,
        }
    }
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Boolean(self.as_bool()), range: as_range(&self.position) }
    }
}

impl NumNode {
    pub fn as_num(&self) -> VosResult<BigDecimal> {
        Ok(BigDecimal::from_str(&self.string)?)
    }
    pub fn as_value(&self) -> VosResult<ValueStatement> {
        Ok(ValueStatement { kind: ValueKind::Number(self.as_num()?), range: as_range(&self.position) })
    }
}

impl NamespaceNode {
    pub fn as_namespace(&self) -> Namespace {
        let mut ns = Namespace::default();
        for id in &self.path {
            ns.push_identifier(id.as_string(), as_range(&id.position))
        }
        ns
    }
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Symbol(self.as_namespace()), range: as_range(&self.position) }
    }
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        let generic = GenericStatement::Arguments { arguments: vec![] };
        Ok(generic)
    }
}