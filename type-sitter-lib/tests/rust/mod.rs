#[doc = concat!("Typed node `", "_declaration_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum DeclarationStatement<'tree> {
    AssociatedType(AssociatedType<'tree>),
    AttributeItem(AttributeItem<'tree>),
    ConstItem(ConstItem<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EnumItem(EnumItem<'tree>),
    ExternCrateDeclaration(ExternCrateDeclaration<'tree>),
    ForeignModItem(ForeignModItem<'tree>),
    FunctionItem(FunctionItem<'tree>),
    FunctionSignatureItem(FunctionSignatureItem<'tree>),
    ImplItem(ImplItem<'tree>),
    InnerAttributeItem(InnerAttributeItem<'tree>),
    LetDeclaration(LetDeclaration<'tree>),
    MacroDefinition(MacroDefinition<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    ModItem(ModItem<'tree>),
    StaticItem(StaticItem<'tree>),
    StructItem(StructItem<'tree>),
    TraitItem(TraitItem<'tree>),
    TypeItem(TypeItem<'tree>),
    UnionItem(UnionItem<'tree>),
    UseDeclaration(UseDeclaration<'tree>),
}
#[automatically_derived]
impl<'tree> DeclarationStatement<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "associated_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn associated_type(self) -> Option<AssociatedType<'tree>> {
        match self {
            Self::AssociatedType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "attribute_item",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn attribute_item(self) -> Option<AttributeItem<'tree>> {
        match self {
            Self::AttributeItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "const_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_item(self) -> Option<ConstItem<'tree>> {
        match self {
            Self::ConstItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "empty_statement",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn empty_statement(self) -> Option<EmptyStatement<'tree>> {
        match self {
            Self::EmptyStatement(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "enum_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn enum_item(self) -> Option<EnumItem<'tree>> {
        match self {
            Self::EnumItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "extern_crate_declaration",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn extern_crate_declaration(self) -> Option<ExternCrateDeclaration<'tree>> {
        match self {
            Self::ExternCrateDeclaration(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "foreign_mod_item",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn foreign_mod_item(self) -> Option<ForeignModItem<'tree>> {
        match self {
            Self::ForeignModItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "function_item",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_item(self) -> Option<FunctionItem<'tree>> {
        match self {
            Self::FunctionItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "function_signature_item",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_signature_item(self) -> Option<FunctionSignatureItem<'tree>> {
        match self {
            Self::FunctionSignatureItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "impl_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn impl_item(self) -> Option<ImplItem<'tree>> {
        match self {
            Self::ImplItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "inner_attribute_item",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn inner_attribute_item(self) -> Option<InnerAttributeItem<'tree>> {
        match self {
            Self::InnerAttributeItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "let_declaration",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn let_declaration(self) -> Option<LetDeclaration<'tree>> {
        match self {
            Self::LetDeclaration(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "macro_definition",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_definition(self) -> Option<MacroDefinition<'tree>> {
        match self {
            Self::MacroDefinition(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "macro_invocation",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "mod_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn mod_item(self) -> Option<ModItem<'tree>> {
        match self {
            Self::ModItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "static_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn static_item(self) -> Option<StaticItem<'tree>> {
        match self {
            Self::StaticItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "struct_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_item(self) -> Option<StructItem<'tree>> {
        match self {
            Self::StructItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "trait_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn trait_item(self) -> Option<TraitItem<'tree>> {
        match self {
            Self::TraitItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "type_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_item(self) -> Option<TypeItem<'tree>> {
        match self {
            Self::TypeItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "union_item", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn union_item(self) -> Option<UnionItem<'tree>> {
        match self {
            Self::UnionItem(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "use_declaration",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn use_declaration(self) -> Option<UseDeclaration<'tree>> {
        match self {
            Self::UseDeclaration(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DeclarationStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "associated_type" => {
                Ok(unsafe {
                    Self::AssociatedType(
                        <AssociatedType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "attribute_item" => {
                Ok(unsafe {
                    Self::AttributeItem(
                        <AttributeItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "const_item" => {
                Ok(unsafe {
                    Self::ConstItem(
                        <ConstItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "empty_statement" => {
                Ok(unsafe {
                    Self::EmptyStatement(
                        <EmptyStatement as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "enum_item" => {
                Ok(unsafe {
                    Self::EnumItem(
                        <EnumItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "extern_crate_declaration" => {
                Ok(unsafe {
                    Self::ExternCrateDeclaration(
                        <ExternCrateDeclaration as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "foreign_mod_item" => {
                Ok(unsafe {
                    Self::ForeignModItem(
                        <ForeignModItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "function_item" => {
                Ok(unsafe {
                    Self::FunctionItem(
                        <FunctionItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "function_signature_item" => {
                Ok(unsafe {
                    Self::FunctionSignatureItem(
                        <FunctionSignatureItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "impl_item" => {
                Ok(unsafe {
                    Self::ImplItem(
                        <ImplItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "inner_attribute_item" => {
                Ok(unsafe {
                    Self::InnerAttributeItem(
                        <InnerAttributeItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "let_declaration" => {
                Ok(unsafe {
                    Self::LetDeclaration(
                        <LetDeclaration as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "macro_definition" => {
                Ok(unsafe {
                    Self::MacroDefinition(
                        <MacroDefinition as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "macro_invocation" => {
                Ok(unsafe {
                    Self::MacroInvocation(
                        <MacroInvocation as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "mod_item" => {
                Ok(unsafe {
                    Self::ModItem(
                        <ModItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "static_item" => {
                Ok(unsafe {
                    Self::StaticItem(
                        <StaticItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "struct_item" => {
                Ok(unsafe {
                    Self::StructItem(
                        <StructItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "trait_item" => {
                Ok(unsafe {
                    Self::TraitItem(
                        <TraitItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "type_item" => {
                Ok(unsafe {
                    Self::TypeItem(
                        <TypeItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "union_item" => {
                Ok(unsafe {
                    Self::UnionItem(
                        <UnionItem as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "use_declaration" => {
                Ok(unsafe {
                    Self::UseDeclaration(
                        <UseDeclaration as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DeclarationStatement<'tree> {
    const KIND: &'static str = "_declaration_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::AssociatedType(x) => x.node(),
            Self::AttributeItem(x) => x.node(),
            Self::ConstItem(x) => x.node(),
            Self::EmptyStatement(x) => x.node(),
            Self::EnumItem(x) => x.node(),
            Self::ExternCrateDeclaration(x) => x.node(),
            Self::ForeignModItem(x) => x.node(),
            Self::FunctionItem(x) => x.node(),
            Self::FunctionSignatureItem(x) => x.node(),
            Self::ImplItem(x) => x.node(),
            Self::InnerAttributeItem(x) => x.node(),
            Self::LetDeclaration(x) => x.node(),
            Self::MacroDefinition(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::ModItem(x) => x.node(),
            Self::StaticItem(x) => x.node(),
            Self::StructItem(x) => x.node(),
            Self::TraitItem(x) => x.node(),
            Self::TypeItem(x) => x.node(),
            Self::UnionItem(x) => x.node(),
            Self::UseDeclaration(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Expression<'tree> {
    Literal(Literal<'tree>),
    ArrayExpression(ArrayExpression<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    AsyncBlock(AsyncBlock<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BreakExpression(BreakExpression<'tree>),
    CallExpression(CallExpression<'tree>),
    ClosureExpression(ClosureExpression<'tree>),
    CompoundAssignmentExpr(CompoundAssignmentExpr<'tree>),
    ConstBlock(ConstBlock<'tree>),
    ContinueExpression(ContinueExpression<'tree>),
    FieldExpression(FieldExpression<'tree>),
    ForExpression(ForExpression<'tree>),
    GenericFunction(GenericFunction<'tree>),
    Identifier(Identifier<'tree>),
    IfExpression(IfExpression<'tree>),
    IndexExpression(IndexExpression<'tree>),
    LoopExpression(LoopExpression<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    MatchExpression(MatchExpression<'tree>),
    Metavariable(Metavariable<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    RangeExpression(RangeExpression<'tree>),
    ReferenceExpression(ReferenceExpression<'tree>),
    ReturnExpression(ReturnExpression<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    _Self(_Self<'tree>),
    StructExpression(StructExpression<'tree>),
    TryExpression(TryExpression<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TypeCastExpression(TypeCastExpression<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnitExpression(UnitExpression<'tree>),
    UnsafeBlock(UnsafeBlock<'tree>),
    WhileExpression(WhileExpression<'tree>),
    YieldExpression(YieldExpression<'tree>),
}
#[automatically_derived]
impl<'tree> Expression<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "_literal", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn literal(self) -> Option<Literal<'tree>> {
        match self {
            Self::Literal(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "array_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array_expression(self) -> Option<ArrayExpression<'tree>> {
        match self {
            Self::ArrayExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "assignment_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn assignment_expression(self) -> Option<AssignmentExpression<'tree>> {
        match self {
            Self::AssignmentExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "async_block", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn async_block(self) -> Option<AsyncBlock<'tree>> {
        match self {
            Self::AsyncBlock(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "await_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn await_expression(self) -> Option<AwaitExpression<'tree>> {
        match self {
            Self::AwaitExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "binary_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn binary_expression(self) -> Option<BinaryExpression<'tree>> {
        match self {
            Self::BinaryExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "block", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn block(self) -> Option<Block<'tree>> {
        match self {
            Self::Block(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "break_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn break_expression(self) -> Option<BreakExpression<'tree>> {
        match self {
            Self::BreakExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "call_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn call_expression(self) -> Option<CallExpression<'tree>> {
        match self {
            Self::CallExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "closure_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn closure_expression(self) -> Option<ClosureExpression<'tree>> {
        match self {
            Self::ClosureExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "compound_assignment_expr",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn compound_assignment_expr(self) -> Option<CompoundAssignmentExpr<'tree>> {
        match self {
            Self::CompoundAssignmentExpr(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "const_block", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_block(self) -> Option<ConstBlock<'tree>> {
        match self {
            Self::ConstBlock(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "continue_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn continue_expression(self) -> Option<ContinueExpression<'tree>> {
        match self {
            Self::ContinueExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "field_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn field_expression(self) -> Option<FieldExpression<'tree>> {
        match self {
            Self::FieldExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "for_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn for_expression(self) -> Option<ForExpression<'tree>> {
        match self {
            Self::ForExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "generic_function",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn generic_function(self) -> Option<GenericFunction<'tree>> {
        match self {
            Self::GenericFunction(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "identifier", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        match self {
            Self::Identifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "if_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn if_expression(self) -> Option<IfExpression<'tree>> {
        match self {
            Self::IfExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "index_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn index_expression(self) -> Option<IndexExpression<'tree>> {
        match self {
            Self::IndexExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "loop_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn loop_expression(self) -> Option<LoopExpression<'tree>> {
        match self {
            Self::LoopExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "macro_invocation",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "match_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn match_expression(self) -> Option<MatchExpression<'tree>> {
        match self {
            Self::MatchExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "metavariable",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn metavariable(self) -> Option<Metavariable<'tree>> {
        match self {
            Self::Metavariable(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "parenthesized_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn parenthesized_expression(self) -> Option<ParenthesizedExpression<'tree>> {
        match self {
            Self::ParenthesizedExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "range_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn range_expression(self) -> Option<RangeExpression<'tree>> {
        match self {
            Self::RangeExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "reference_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_expression(self) -> Option<ReferenceExpression<'tree>> {
        match self {
            Self::ReferenceExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "return_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn return_expression(self) -> Option<ReturnExpression<'tree>> {
        match self {
            Self::ReturnExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "scoped_identifier",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
        match self {
            Self::ScopedIdentifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "self", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn _self(self) -> Option<_Self<'tree>> {
        match self {
            Self::_Self(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "struct_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_expression(self) -> Option<StructExpression<'tree>> {
        match self {
            Self::StructExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "try_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn try_expression(self) -> Option<TryExpression<'tree>> {
        match self {
            Self::TryExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "tuple_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_expression(self) -> Option<TupleExpression<'tree>> {
        match self {
            Self::TupleExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "type_cast_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_cast_expression(self) -> Option<TypeCastExpression<'tree>> {
        match self {
            Self::TypeCastExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "unary_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unary_expression(self) -> Option<UnaryExpression<'tree>> {
        match self {
            Self::UnaryExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "unit_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unit_expression(self) -> Option<UnitExpression<'tree>> {
        match self {
            Self::UnitExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "unsafe_block",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unsafe_block(self) -> Option<UnsafeBlock<'tree>> {
        match self {
            Self::UnsafeBlock(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "while_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn while_expression(self) -> Option<WhileExpression<'tree>> {
        match self {
            Self::WhileExpression(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "yield_expression",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn yield_expression(self) -> Option<YieldExpression<'tree>> {
        match self {
            Self::YieldExpression(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Expression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "_literal" => {
                Ok(unsafe {
                    Self::Literal(
                        <Literal as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "array_expression" => {
                Ok(unsafe {
                    Self::ArrayExpression(
                        <ArrayExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "assignment_expression" => {
                Ok(unsafe {
                    Self::AssignmentExpression(
                        <AssignmentExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "async_block" => {
                Ok(unsafe {
                    Self::AsyncBlock(
                        <AsyncBlock as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "await_expression" => {
                Ok(unsafe {
                    Self::AwaitExpression(
                        <AwaitExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "binary_expression" => {
                Ok(unsafe {
                    Self::BinaryExpression(
                        <BinaryExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "block" => {
                Ok(unsafe {
                    Self::Block(
                        <Block as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "break_expression" => {
                Ok(unsafe {
                    Self::BreakExpression(
                        <BreakExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "call_expression" => {
                Ok(unsafe {
                    Self::CallExpression(
                        <CallExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "closure_expression" => {
                Ok(unsafe {
                    Self::ClosureExpression(
                        <ClosureExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "compound_assignment_expr" => {
                Ok(unsafe {
                    Self::CompoundAssignmentExpr(
                        <CompoundAssignmentExpr as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "const_block" => {
                Ok(unsafe {
                    Self::ConstBlock(
                        <ConstBlock as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "continue_expression" => {
                Ok(unsafe {
                    Self::ContinueExpression(
                        <ContinueExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "field_expression" => {
                Ok(unsafe {
                    Self::FieldExpression(
                        <FieldExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "for_expression" => {
                Ok(unsafe {
                    Self::ForExpression(
                        <ForExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "generic_function" => {
                Ok(unsafe {
                    Self::GenericFunction(
                        <GenericFunction as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "identifier" => {
                Ok(unsafe {
                    Self::Identifier(
                        <Identifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "if_expression" => {
                Ok(unsafe {
                    Self::IfExpression(
                        <IfExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "index_expression" => {
                Ok(unsafe {
                    Self::IndexExpression(
                        <IndexExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "loop_expression" => {
                Ok(unsafe {
                    Self::LoopExpression(
                        <LoopExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "macro_invocation" => {
                Ok(unsafe {
                    Self::MacroInvocation(
                        <MacroInvocation as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "match_expression" => {
                Ok(unsafe {
                    Self::MatchExpression(
                        <MatchExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "metavariable" => {
                Ok(unsafe {
                    Self::Metavariable(
                        <Metavariable as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "parenthesized_expression" => {
                Ok(unsafe {
                    Self::ParenthesizedExpression(
                        <ParenthesizedExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "range_expression" => {
                Ok(unsafe {
                    Self::RangeExpression(
                        <RangeExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "reference_expression" => {
                Ok(unsafe {
                    Self::ReferenceExpression(
                        <ReferenceExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "return_expression" => {
                Ok(unsafe {
                    Self::ReturnExpression(
                        <ReturnExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "scoped_identifier" => {
                Ok(unsafe {
                    Self::ScopedIdentifier(
                        <ScopedIdentifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "self" => {
                Ok(unsafe {
                    Self::_Self(
                        <_Self as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "struct_expression" => {
                Ok(unsafe {
                    Self::StructExpression(
                        <StructExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "try_expression" => {
                Ok(unsafe {
                    Self::TryExpression(
                        <TryExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "tuple_expression" => {
                Ok(unsafe {
                    Self::TupleExpression(
                        <TupleExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "type_cast_expression" => {
                Ok(unsafe {
                    Self::TypeCastExpression(
                        <TypeCastExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "unary_expression" => {
                Ok(unsafe {
                    Self::UnaryExpression(
                        <UnaryExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "unit_expression" => {
                Ok(unsafe {
                    Self::UnitExpression(
                        <UnitExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "unsafe_block" => {
                Ok(unsafe {
                    Self::UnsafeBlock(
                        <UnsafeBlock as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "while_expression" => {
                Ok(unsafe {
                    Self::WhileExpression(
                        <WhileExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "yield_expression" => {
                Ok(unsafe {
                    Self::YieldExpression(
                        <YieldExpression as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Expression<'tree> {
    const KIND: &'static str = "_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::Literal(x) => x.node(),
            Self::ArrayExpression(x) => x.node(),
            Self::AssignmentExpression(x) => x.node(),
            Self::AsyncBlock(x) => x.node(),
            Self::AwaitExpression(x) => x.node(),
            Self::BinaryExpression(x) => x.node(),
            Self::Block(x) => x.node(),
            Self::BreakExpression(x) => x.node(),
            Self::CallExpression(x) => x.node(),
            Self::ClosureExpression(x) => x.node(),
            Self::CompoundAssignmentExpr(x) => x.node(),
            Self::ConstBlock(x) => x.node(),
            Self::ContinueExpression(x) => x.node(),
            Self::FieldExpression(x) => x.node(),
            Self::ForExpression(x) => x.node(),
            Self::GenericFunction(x) => x.node(),
            Self::Identifier(x) => x.node(),
            Self::IfExpression(x) => x.node(),
            Self::IndexExpression(x) => x.node(),
            Self::LoopExpression(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::MatchExpression(x) => x.node(),
            Self::Metavariable(x) => x.node(),
            Self::ParenthesizedExpression(x) => x.node(),
            Self::RangeExpression(x) => x.node(),
            Self::ReferenceExpression(x) => x.node(),
            Self::ReturnExpression(x) => x.node(),
            Self::ScopedIdentifier(x) => x.node(),
            Self::_Self(x) => x.node(),
            Self::StructExpression(x) => x.node(),
            Self::TryExpression(x) => x.node(),
            Self::TupleExpression(x) => x.node(),
            Self::TypeCastExpression(x) => x.node(),
            Self::UnaryExpression(x) => x.node(),
            Self::UnitExpression(x) => x.node(),
            Self::UnsafeBlock(x) => x.node(),
            Self::WhileExpression(x) => x.node(),
            Self::YieldExpression(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Literal<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
#[automatically_derived]
impl<'tree> Literal<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "boolean_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
        match self {
            Self::BooleanLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "char_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn char_literal(self) -> Option<CharLiteral<'tree>> {
        match self {
            Self::CharLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "float_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
        match self {
            Self::FloatLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "integer_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
        match self {
            Self::IntegerLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "raw_string_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
        match self {
            Self::RawStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "string_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_literal(self) -> Option<StringLiteral<'tree>> {
        match self {
            Self::StringLiteral(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Literal<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => {
                Ok(unsafe {
                    Self::BooleanLiteral(
                        <BooleanLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "char_literal" => {
                Ok(unsafe {
                    Self::CharLiteral(
                        <CharLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "float_literal" => {
                Ok(unsafe {
                    Self::FloatLiteral(
                        <FloatLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "integer_literal" => {
                Ok(unsafe {
                    Self::IntegerLiteral(
                        <IntegerLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "raw_string_literal" => {
                Ok(unsafe {
                    Self::RawStringLiteral(
                        <RawStringLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "string_literal" => {
                Ok(unsafe {
                    Self::StringLiteral(
                        <StringLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Literal<'tree> {
    const KIND: &'static str = "_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node(),
            Self::CharLiteral(x) => x.node(),
            Self::FloatLiteral(x) => x.node(),
            Self::IntegerLiteral(x) => x.node(),
            Self::RawStringLiteral(x) => x.node(),
            Self::StringLiteral(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "_literal_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum LiteralPattern<'tree> {
    BooleanLiteral(BooleanLiteral<'tree>),
    CharLiteral(CharLiteral<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    NegativeLiteral(NegativeLiteral<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    StringLiteral(StringLiteral<'tree>),
}
#[automatically_derived]
impl<'tree> LiteralPattern<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "boolean_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn boolean_literal(self) -> Option<BooleanLiteral<'tree>> {
        match self {
            Self::BooleanLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "char_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn char_literal(self) -> Option<CharLiteral<'tree>> {
        match self {
            Self::CharLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "float_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn float_literal(self) -> Option<FloatLiteral<'tree>> {
        match self {
            Self::FloatLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "integer_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn integer_literal(self) -> Option<IntegerLiteral<'tree>> {
        match self {
            Self::IntegerLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "negative_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn negative_literal(self) -> Option<NegativeLiteral<'tree>> {
        match self {
            Self::NegativeLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "raw_string_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn raw_string_literal(self) -> Option<RawStringLiteral<'tree>> {
        match self {
            Self::RawStringLiteral(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "string_literal",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn string_literal(self) -> Option<StringLiteral<'tree>> {
        match self {
            Self::StringLiteral(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LiteralPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "boolean_literal" => {
                Ok(unsafe {
                    Self::BooleanLiteral(
                        <BooleanLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "char_literal" => {
                Ok(unsafe {
                    Self::CharLiteral(
                        <CharLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "float_literal" => {
                Ok(unsafe {
                    Self::FloatLiteral(
                        <FloatLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "integer_literal" => {
                Ok(unsafe {
                    Self::IntegerLiteral(
                        <IntegerLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "negative_literal" => {
                Ok(unsafe {
                    Self::NegativeLiteral(
                        <NegativeLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "raw_string_literal" => {
                Ok(unsafe {
                    Self::RawStringLiteral(
                        <RawStringLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "string_literal" => {
                Ok(unsafe {
                    Self::StringLiteral(
                        <StringLiteral as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LiteralPattern<'tree> {
    const KIND: &'static str = "_literal_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::BooleanLiteral(x) => x.node(),
            Self::CharLiteral(x) => x.node(),
            Self::FloatLiteral(x) => x.node(),
            Self::IntegerLiteral(x) => x.node(),
            Self::NegativeLiteral(x) => x.node(),
            Self::RawStringLiteral(x) => x.node(),
            Self::StringLiteral(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Pattern<'tree> {
    __(__<'tree>),
    LiteralPattern(LiteralPattern<'tree>),
    CapturedPattern(CapturedPattern<'tree>),
    ConstBlock(ConstBlock<'tree>),
    Identifier(Identifier<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    MutPattern(MutPattern<'tree>),
    OrPattern(OrPattern<'tree>),
    RangePattern(RangePattern<'tree>),
    RefPattern(RefPattern<'tree>),
    ReferencePattern(ReferencePattern<'tree>),
    RemainingFieldPattern(RemainingFieldPattern<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    SlicePattern(SlicePattern<'tree>),
    StructPattern(StructPattern<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleStructPattern(TupleStructPattern<'tree>),
}
#[automatically_derived]
impl<'tree> Pattern<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "_", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn __(self) -> Option<__<'tree>> {
        match self {
            Self::__(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "_literal_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn literal_pattern(self) -> Option<LiteralPattern<'tree>> {
        match self {
            Self::LiteralPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "captured_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn captured_pattern(self) -> Option<CapturedPattern<'tree>> {
        match self {
            Self::CapturedPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "const_block", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn const_block(self) -> Option<ConstBlock<'tree>> {
        match self {
            Self::ConstBlock(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "identifier", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn identifier(self) -> Option<Identifier<'tree>> {
        match self {
            Self::Identifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "macro_invocation",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "mut_pattern", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn mut_pattern(self) -> Option<MutPattern<'tree>> {
        match self {
            Self::MutPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "or_pattern", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn or_pattern(self) -> Option<OrPattern<'tree>> {
        match self {
            Self::OrPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "range_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn range_pattern(self) -> Option<RangePattern<'tree>> {
        match self {
            Self::RangePattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "ref_pattern", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn ref_pattern(self) -> Option<RefPattern<'tree>> {
        match self {
            Self::RefPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "reference_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_pattern(self) -> Option<ReferencePattern<'tree>> {
        match self {
            Self::ReferencePattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "remaining_field_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn remaining_field_pattern(self) -> Option<RemainingFieldPattern<'tree>> {
        match self {
            Self::RemainingFieldPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "scoped_identifier",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_identifier(self) -> Option<ScopedIdentifier<'tree>> {
        match self {
            Self::ScopedIdentifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "slice_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn slice_pattern(self) -> Option<SlicePattern<'tree>> {
        match self {
            Self::SlicePattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "struct_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn struct_pattern(self) -> Option<StructPattern<'tree>> {
        match self {
            Self::StructPattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "tuple_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_pattern(self) -> Option<TuplePattern<'tree>> {
        match self {
            Self::TuplePattern(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "tuple_struct_pattern",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_struct_pattern(self) -> Option<TupleStructPattern<'tree>> {
        match self {
            Self::TupleStructPattern(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Pattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "_" => {
                Ok(unsafe {
                    Self::__(
                        <__ as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "_literal_pattern" => {
                Ok(unsafe {
                    Self::LiteralPattern(
                        <LiteralPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "captured_pattern" => {
                Ok(unsafe {
                    Self::CapturedPattern(
                        <CapturedPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "const_block" => {
                Ok(unsafe {
                    Self::ConstBlock(
                        <ConstBlock as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "identifier" => {
                Ok(unsafe {
                    Self::Identifier(
                        <Identifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "macro_invocation" => {
                Ok(unsafe {
                    Self::MacroInvocation(
                        <MacroInvocation as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "mut_pattern" => {
                Ok(unsafe {
                    Self::MutPattern(
                        <MutPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "or_pattern" => {
                Ok(unsafe {
                    Self::OrPattern(
                        <OrPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "range_pattern" => {
                Ok(unsafe {
                    Self::RangePattern(
                        <RangePattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "ref_pattern" => {
                Ok(unsafe {
                    Self::RefPattern(
                        <RefPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "reference_pattern" => {
                Ok(unsafe {
                    Self::ReferencePattern(
                        <ReferencePattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "remaining_field_pattern" => {
                Ok(unsafe {
                    Self::RemainingFieldPattern(
                        <RemainingFieldPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "scoped_identifier" => {
                Ok(unsafe {
                    Self::ScopedIdentifier(
                        <ScopedIdentifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "slice_pattern" => {
                Ok(unsafe {
                    Self::SlicePattern(
                        <SlicePattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "struct_pattern" => {
                Ok(unsafe {
                    Self::StructPattern(
                        <StructPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "tuple_pattern" => {
                Ok(unsafe {
                    Self::TuplePattern(
                        <TuplePattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "tuple_struct_pattern" => {
                Ok(unsafe {
                    Self::TupleStructPattern(
                        <TupleStructPattern as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Pattern<'tree> {
    const KIND: &'static str = "_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::__(x) => x.node(),
            Self::LiteralPattern(x) => x.node(),
            Self::CapturedPattern(x) => x.node(),
            Self::ConstBlock(x) => x.node(),
            Self::Identifier(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::MutPattern(x) => x.node(),
            Self::OrPattern(x) => x.node(),
            Self::RangePattern(x) => x.node(),
            Self::RefPattern(x) => x.node(),
            Self::ReferencePattern(x) => x.node(),
            Self::RemainingFieldPattern(x) => x.node(),
            Self::ScopedIdentifier(x) => x.node(),
            Self::SlicePattern(x) => x.node(),
            Self::StructPattern(x) => x.node(),
            Self::TuplePattern(x) => x.node(),
            Self::TupleStructPattern(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Type<'tree> {
    AbstractType(AbstractType<'tree>),
    ArrayType(ArrayType<'tree>),
    BoundedType(BoundedType<'tree>),
    DynamicType(DynamicType<'tree>),
    EmptyType(EmptyType<'tree>),
    FunctionType(FunctionType<'tree>),
    GenericType(GenericType<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    Metavariable(Metavariable<'tree>),
    PointerType(PointerType<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    ReferenceType(ReferenceType<'tree>),
    ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
    TupleType(TupleType<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    UnitType(UnitType<'tree>),
}
#[automatically_derived]
impl<'tree> Type<'tree> {
    #[doc = concat!(
        "Returns the node if it is of kind `", "abstract_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn abstract_type(self) -> Option<AbstractType<'tree>> {
        match self {
            Self::AbstractType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "array_type", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn array_type(self) -> Option<ArrayType<'tree>> {
        match self {
            Self::ArrayType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "bounded_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn bounded_type(self) -> Option<BoundedType<'tree>> {
        match self {
            Self::BoundedType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "dynamic_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn dynamic_type(self) -> Option<DynamicType<'tree>> {
        match self {
            Self::DynamicType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "empty_type", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn empty_type(self) -> Option<EmptyType<'tree>> {
        match self {
            Self::EmptyType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "function_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn function_type(self) -> Option<FunctionType<'tree>> {
        match self {
            Self::FunctionType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "generic_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn generic_type(self) -> Option<GenericType<'tree>> {
        match self {
            Self::GenericType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "macro_invocation",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn macro_invocation(self) -> Option<MacroInvocation<'tree>> {
        match self {
            Self::MacroInvocation(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "metavariable",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn metavariable(self) -> Option<Metavariable<'tree>> {
        match self {
            Self::Metavariable(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "pointer_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn pointer_type(self) -> Option<PointerType<'tree>> {
        match self {
            Self::PointerType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "primitive_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn primitive_type(self) -> Option<PrimitiveType<'tree>> {
        match self {
            Self::PrimitiveType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "reference_type",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn reference_type(self) -> Option<ReferenceType<'tree>> {
        match self {
            Self::ReferenceType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "scoped_type_identifier",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn scoped_type_identifier(self) -> Option<ScopedTypeIdentifier<'tree>> {
        match self {
            Self::ScopedTypeIdentifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "tuple_type", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn tuple_type(self) -> Option<TupleType<'tree>> {
        match self {
            Self::TupleType(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "type_identifier",
        "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn type_identifier(self) -> Option<TypeIdentifier<'tree>> {
        match self {
            Self::TypeIdentifier(x) => Some(x),
            _ => None,
        }
    }
    #[doc = concat!(
        "Returns the node if it is of kind `", "unit_type", "`, otherwise returns None"
    )]
    #[inline]
    #[allow(unused, non_snake_case)]
    pub fn unit_type(self) -> Option<UnitType<'tree>> {
        match self {
            Self::UnitType(x) => Some(x),
            _ => None,
        }
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Type<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        match node.kind() {
            "abstract_type" => {
                Ok(unsafe {
                    Self::AbstractType(
                        <AbstractType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "array_type" => {
                Ok(unsafe {
                    Self::ArrayType(
                        <ArrayType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "bounded_type" => {
                Ok(unsafe {
                    Self::BoundedType(
                        <BoundedType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "dynamic_type" => {
                Ok(unsafe {
                    Self::DynamicType(
                        <DynamicType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "empty_type" => {
                Ok(unsafe {
                    Self::EmptyType(
                        <EmptyType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "function_type" => {
                Ok(unsafe {
                    Self::FunctionType(
                        <FunctionType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "generic_type" => {
                Ok(unsafe {
                    Self::GenericType(
                        <GenericType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "macro_invocation" => {
                Ok(unsafe {
                    Self::MacroInvocation(
                        <MacroInvocation as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "metavariable" => {
                Ok(unsafe {
                    Self::Metavariable(
                        <Metavariable as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "pointer_type" => {
                Ok(unsafe {
                    Self::PointerType(
                        <PointerType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "primitive_type" => {
                Ok(unsafe {
                    Self::PrimitiveType(
                        <PrimitiveType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "reference_type" => {
                Ok(unsafe {
                    Self::ReferenceType(
                        <ReferenceType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "scoped_type_identifier" => {
                Ok(unsafe {
                    Self::ScopedTypeIdentifier(
                        <ScopedTypeIdentifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "tuple_type" => {
                Ok(unsafe {
                    Self::TupleType(
                        <TupleType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "type_identifier" => {
                Ok(unsafe {
                    Self::TypeIdentifier(
                        <TypeIdentifier as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            "unit_type" => {
                Ok(unsafe {
                    Self::UnitType(
                        <UnitType as type_sitter_lib::TypedNode<
                            'tree,
                        >>::from_node_unchecked(node),
                    )
                })
            }
            _ => {
                Err(type_sitter_lib::IncorrectKind {
                    node,
                    kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
                })
            }
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Type<'tree> {
    const KIND: &'static str = "_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        match self {
            Self::AbstractType(x) => x.node(),
            Self::ArrayType(x) => x.node(),
            Self::BoundedType(x) => x.node(),
            Self::DynamicType(x) => x.node(),
            Self::EmptyType(x) => x.node(),
            Self::FunctionType(x) => x.node(),
            Self::GenericType(x) => x.node(),
            Self::MacroInvocation(x) => x.node(),
            Self::Metavariable(x) => x.node(),
            Self::PointerType(x) => x.node(),
            Self::PrimitiveType(x) => x.node(),
            Self::ReferenceType(x) => x.node(),
            Self::ScopedTypeIdentifier(x) => x.node(),
            Self::TupleType(x) => x.node(),
            Self::TypeIdentifier(x) => x.node(),
            Self::UnitType(x) => x.node(),
        }
    }
}
#[doc = concat!("Typed node `", "abstract_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AbstractType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AbstractType<'tree> {
    #[doc = concat!("Get the field `", "trait", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either4<
            FunctionType<'tree>,
            GenericType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .map(
                <type_sitter_lib::Either4<
                    FunctionType<'tree>,
                    GenericType<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AbstractType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "abstract_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AbstractType<'tree> {
    const KIND: &'static str = "abstract_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "arguments", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Arguments<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Arguments<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Arguments<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "arguments" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Arguments<'tree> {
    const KIND: &'static str = "arguments";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "array_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArrayExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArrayExpression<'tree> {
    #[doc = concat!("Get the field `", "length", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn length(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("length")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ArrayExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArrayExpression<'tree> {
    const KIND: &'static str = "array_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "array_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ArrayType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ArrayType<'tree> {
    #[doc = concat!("Get the field `", "element", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn element(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("element")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "length", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn length(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("length")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ArrayType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "array_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ArrayType<'tree> {
    const KIND: &'static str = "array_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "assignment_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AssignmentExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AssignmentExpression<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AssignmentExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "assignment_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AssignmentExpression<'tree> {
    const KIND: &'static str = "assignment_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "associated_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AssociatedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AssociatedType<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AssociatedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "associated_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AssociatedType<'tree> {
    const KIND: &'static str = "associated_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "async_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AsyncBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AsyncBlock<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .named_child(0)
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AsyncBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "async_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AsyncBlock<'tree> {
    const KIND: &'static str = "async_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "attribute", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Attribute<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Attribute<'tree> {
    #[doc = concat!("Get the field `", "arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn arguments(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TokenTree<'tree>>> {
        self.0
            .child_by_field_name("arguments")
            .map(<TokenTree<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either6<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            _Self<'tree>,
            _Super<'tree>,
        >,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either6<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Attribute<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Attribute<'tree> {
    const KIND: &'static str = "attribute";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "attribute_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AttributeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AttributeItem<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self.0
            .named_child(0)
            .map(<Attribute<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AttributeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "attribute_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AttributeItem<'tree> {
    const KIND: &'static str = "attribute_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "await_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AwaitExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AwaitExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AwaitExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "await_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AwaitExpression<'tree> {
    const KIND: &'static str = "await_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "base_field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BaseFieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BaseFieldInitializer<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BaseFieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "base_field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BaseFieldInitializer<'tree> {
    const KIND: &'static str = "base_field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "binary_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BinaryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BinaryExpression<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "operator", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn operator(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either18<
            NotEq<'tree>,
            Mod<'tree>,
            And<'tree>,
            AndAnd<'tree>,
            Mul<'tree>,
            Add<'tree>,
            Sub<'tree>,
            Div<'tree>,
            Lt<'tree>,
            LtLt<'tree>,
            LtEq<'tree>,
            EqEq<'tree>,
            Gt<'tree>,
            GtEq<'tree>,
            GtGt<'tree>,
            BitXor<'tree>,
            Or<'tree>,
            OrOr<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("operator")
            .map(
                <type_sitter_lib::Either18<
                    NotEq<'tree>,
                    Mod<'tree>,
                    And<'tree>,
                    AndAnd<'tree>,
                    Mul<'tree>,
                    Add<'tree>,
                    Sub<'tree>,
                    Div<'tree>,
                    Lt<'tree>,
                    LtLt<'tree>,
                    LtEq<'tree>,
                    EqEq<'tree>,
                    Gt<'tree>,
                    GtEq<'tree>,
                    GtGt<'tree>,
                    BitXor<'tree>,
                    Or<'tree>,
                    OrOr<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BinaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "binary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BinaryExpression<'tree> {
    const KIND: &'static str = "binary_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Block<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Block<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    DeclarationStatement<'tree>,
                    Expression<'tree>,
                    ExpressionStatement<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        DeclarationStatement<'tree>,
                        Expression<'tree>,
                        ExpressionStatement<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    DeclarationStatement<'tree>,
                    Expression<'tree>,
                    ExpressionStatement<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        DeclarationStatement<'tree>,
                        Expression<'tree>,
                        ExpressionStatement<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Block<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Block<'tree> {
    const KIND: &'static str = "block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "boolean_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BooleanLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BooleanLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BooleanLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "boolean_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BooleanLiteral<'tree> {
    const KIND: &'static str = "boolean_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "bounded_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BoundedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BoundedType<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Type<'tree>, Lifetime<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Type<'tree>, Lifetime<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Type<'tree>, Lifetime<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Type<'tree>, Lifetime<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BoundedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bounded_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BoundedType<'tree> {
    const KIND: &'static str = "bounded_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "bracketed_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BracketedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BracketedType<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Type<'tree>, QualifiedType<'tree>>,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either2<
                    Type<'tree>,
                    QualifiedType<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BracketedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "bracketed_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BracketedType<'tree> {
    const KIND: &'static str = "bracketed_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "break_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BreakExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BreakExpression<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, LoopLabel<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, LoopLabel<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, LoopLabel<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, LoopLabel<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BreakExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BreakExpression<'tree> {
    const KIND: &'static str = "break_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "call_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CallExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CallExpression<'tree> {
    #[doc = concat!("Get the field `", "arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn arguments(&self) -> type_sitter_lib::NodeResult<'tree, Arguments<'tree>> {
        self.0
            .child_by_field_name("arguments")
            .map(<Arguments<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "function", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn function(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either![
            Literal < 'tree >, ArrayExpression < 'tree >, AssignmentExpression < 'tree >,
            AsyncBlock < 'tree >, AwaitExpression < 'tree >, BinaryExpression < 'tree >,
            Block < 'tree >, BreakExpression < 'tree >, CallExpression < 'tree >,
            ClosureExpression < 'tree >, CompoundAssignmentExpr < 'tree >, ConstBlock <
            'tree >, ContinueExpression < 'tree >, FieldExpression < 'tree >,
            ForExpression < 'tree >, GenericFunction < 'tree >, Identifier < 'tree >,
            IfExpression < 'tree >, IndexExpression < 'tree >, LoopExpression < 'tree >,
            MacroInvocation < 'tree >, MatchExpression < 'tree >, Metavariable < 'tree >,
            ParenthesizedExpression < 'tree >, ReferenceExpression < 'tree >,
            ReturnExpression < 'tree >, ScopedIdentifier < 'tree >, _Self < 'tree >,
            StructExpression < 'tree >, TryExpression < 'tree >, TupleExpression < 'tree
            >, TypeCastExpression < 'tree >, UnaryExpression < 'tree >, UnitExpression <
            'tree >, UnsafeBlock < 'tree >, WhileExpression < 'tree >, YieldExpression <
            'tree >
        ],
    > {
        self.0
            .child_by_field_name("function")
            .map(
                <type_sitter_lib::Either![
                    Literal < 'tree >, ArrayExpression < 'tree >, AssignmentExpression <
                    'tree >, AsyncBlock < 'tree >, AwaitExpression < 'tree >,
                    BinaryExpression < 'tree >, Block < 'tree >, BreakExpression < 'tree
                    >, CallExpression < 'tree >, ClosureExpression < 'tree >,
                    CompoundAssignmentExpr < 'tree >, ConstBlock < 'tree >,
                    ContinueExpression < 'tree >, FieldExpression < 'tree >,
                    ForExpression < 'tree >, GenericFunction < 'tree >, Identifier <
                    'tree >, IfExpression < 'tree >, IndexExpression < 'tree >,
                    LoopExpression < 'tree >, MacroInvocation < 'tree >, MatchExpression
                    < 'tree >, Metavariable < 'tree >, ParenthesizedExpression < 'tree >,
                    ReferenceExpression < 'tree >, ReturnExpression < 'tree >,
                    ScopedIdentifier < 'tree >, _Self < 'tree >, StructExpression < 'tree
                    >, TryExpression < 'tree >, TupleExpression < 'tree >,
                    TypeCastExpression < 'tree >, UnaryExpression < 'tree >,
                    UnitExpression < 'tree >, UnsafeBlock < 'tree >, WhileExpression <
                    'tree >, YieldExpression < 'tree >
                ] as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CallExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "call_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CallExpression<'tree> {
    const KIND: &'static str = "call_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "captured_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CapturedPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CapturedPattern<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CapturedPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "captured_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CapturedPattern<'tree> {
    const KIND: &'static str = "captured_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "closure_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ClosureExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ClosureExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, ClosureParameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .map(<ClosureParameters<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ClosureExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ClosureExpression<'tree> {
    const KIND: &'static str = "closure_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "closure_parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ClosureParameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ClosureParameters<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, Parameter<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, Parameter<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, Parameter<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, Parameter<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ClosureParameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "closure_parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ClosureParameters<'tree> {
    const KIND: &'static str = "closure_parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "compound_assignment_expr", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CompoundAssignmentExpr<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CompoundAssignmentExpr<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("left")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "operator", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn operator(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either10<
            ModEq<'tree>,
            AndEq<'tree>,
            MulEq<'tree>,
            AddEq<'tree>,
            SubEq<'tree>,
            DivEq<'tree>,
            LtLtEq<'tree>,
            GtGtEq<'tree>,
            BitXorEq<'tree>,
            OrEq<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("operator")
            .map(
                <type_sitter_lib::Either10<
                    ModEq<'tree>,
                    AndEq<'tree>,
                    MulEq<'tree>,
                    AddEq<'tree>,
                    SubEq<'tree>,
                    DivEq<'tree>,
                    LtLtEq<'tree>,
                    GtGtEq<'tree>,
                    BitXorEq<'tree>,
                    OrEq<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("right")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CompoundAssignmentExpr<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "compound_assignment_expr" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CompoundAssignmentExpr<'tree> {
    const KIND: &'static str = "compound_assignment_expr";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "const_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstBlock<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstBlock<'tree> {
    const KIND: &'static str = "const_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "const_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstItem<'tree> {
    const KIND: &'static str = "const_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "const_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstParameter<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstParameter<'tree> {
    const KIND: &'static str = "const_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "constrained_type_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConstrainedTypeParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ConstrainedTypeParameter<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Lifetime<'tree>, TypeIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("left")
            .map(
                <type_sitter_lib::Either2<
                    Lifetime<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ConstrainedTypeParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "constrained_type_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ConstrainedTypeParameter<'tree> {
    const KIND: &'static str = "constrained_type_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "continue_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ContinueExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ContinueExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.named_child(0).map(<LoopLabel<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ContinueExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ContinueExpression<'tree> {
    const KIND: &'static str = "continue_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DeclarationList<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    DeclarationStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, DeclarationStatement<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    DeclarationStatement<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DeclarationList<'tree> {
    const KIND: &'static str = "declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "dynamic_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DynamicType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DynamicType<'tree> {
    #[doc = concat!("Get the field `", "trait", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either4<
            FunctionType<'tree>,
            GenericType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .map(
                <type_sitter_lib::Either4<
                    FunctionType<'tree>,
                    GenericType<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DynamicType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dynamic_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DynamicType<'tree> {
    const KIND: &'static str = "dynamic_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "else_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ElseClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ElseClause<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Block<'tree>, IfExpression<'tree>>,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either2<
                    Block<'tree>,
                    IfExpression<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ElseClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ElseClause<'tree> {
    const KIND: &'static str = "else_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "empty_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EmptyStatement<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EmptyStatement<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EmptyStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EmptyStatement<'tree> {
    const KIND: &'static str = "empty_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "empty_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EmptyType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EmptyType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EmptyType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "empty_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EmptyType<'tree> {
    const KIND: &'static str = "empty_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "enum_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, EnumVariantList<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<EnumVariantList<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumItem<'tree> {
    const KIND: &'static str = "enum_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "enum_variant", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumVariant<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumVariant<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either2<
                FieldDeclarationList<'tree>,
                OrderedFieldDeclarationList<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("body")
            .map(
                <type_sitter_lib::Either2<
                    FieldDeclarationList<'tree>,
                    OrderedFieldDeclarationList<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumVariant<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumVariant<'tree> {
    const KIND: &'static str = "enum_variant";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "enum_variant_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EnumVariantList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EnumVariantList<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<AttributeItem<'tree>, EnumVariant<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EnumVariantList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum_variant_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EnumVariantList<'tree> {
    const KIND: &'static str = "enum_variant_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "expression_statement", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExpressionStatement<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExpressionStatement<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExpressionStatement<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expression_statement" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExpressionStatement<'tree> {
    const KIND: &'static str = "expression_statement";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "extern_crate_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExternCrateDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExternCrateDeclaration<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Identifier<'tree>>> {
        self.0
            .child_by_field_name("alias")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Crate<'tree>, VisibilityModifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExternCrateDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_crate_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExternCrateDeclaration<'tree> {
    const KIND: &'static str = "extern_crate_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "extern_modifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ExternModifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ExternModifier<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, StringLiteral<'tree>>> {
        self.0.named_child(0).map(<StringLiteral<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ExternModifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern_modifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ExternModifier<'tree> {
    const KIND: &'static str = "extern_modifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclaration<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<FieldIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclaration<'tree> {
    const KIND: &'static str = "field_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldDeclarationList<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, FieldDeclaration<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        AttributeItem<'tree>,
                        FieldDeclaration<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, FieldDeclaration<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        AttributeItem<'tree>,
                        FieldDeclaration<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldDeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldDeclarationList<'tree> {
    const KIND: &'static str = "field_declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldExpression<'tree> {
    #[doc = concat!("Get the field `", "field", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn field(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<FieldIdentifier<'tree>, IntegerLiteral<'tree>>,
    > {
        self.0
            .child_by_field_name("field")
            .map(
                <type_sitter_lib::Either2<
                    FieldIdentifier<'tree>,
                    IntegerLiteral<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldExpression<'tree> {
    const KIND: &'static str = "field_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldInitializer<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, FieldIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<FieldIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, AttributeItem<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldInitializer<'tree> {
    const KIND: &'static str = "field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_initializer_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldInitializerList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldInitializerList<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    BaseFieldInitializer<'tree>,
                    FieldInitializer<'tree>,
                    ShorthandFieldInitializer<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        BaseFieldInitializer<'tree>,
                        FieldInitializer<'tree>,
                        ShorthandFieldInitializer<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    BaseFieldInitializer<'tree>,
                    FieldInitializer<'tree>,
                    ShorthandFieldInitializer<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        BaseFieldInitializer<'tree>,
                        FieldInitializer<'tree>,
                        ShorthandFieldInitializer<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldInitializerList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_initializer_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldInitializerList<'tree> {
    const KIND: &'static str = "field_initializer_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldPattern<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<FieldIdentifier<'tree>, ShorthandFieldIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .map(
                <type_sitter_lib::Either2<
                    FieldIdentifier<'tree>,
                    ShorthandFieldIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> Option<type_sitter_lib::NodeResult<'tree, Pattern<'tree>>> {
        self.0
            .child_by_field_name("pattern")
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.named_child(0).map(<MutableSpecifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldPattern<'tree> {
    const KIND: &'static str = "field_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "for_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.named_child(0).map(<LoopLabel<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForExpression<'tree> {
    const KIND: &'static str = "for_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "for_lifetimes", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForLifetimes<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForLifetimes<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Lifetime<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Lifetime<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Lifetime<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForLifetimes<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for_lifetimes" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForLifetimes<'tree> {
    const KIND: &'static str = "for_lifetimes";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "foreign_mod_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ForeignModItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ForeignModItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    ExternModifier<'tree>,
                    VisibilityModifier<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        ExternModifier<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    ExternModifier<'tree>,
                    VisibilityModifier<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        ExternModifier<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ForeignModItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "foreign_mod_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ForeignModItem<'tree> {
    const KIND: &'static str = "foreign_mod_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "fragment_specifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FragmentSpecifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FragmentSpecifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FragmentSpecifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fragment_specifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FragmentSpecifier<'tree> {
    const KIND: &'static str = "fragment_specifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "function_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Identifier<'tree>, Metavariable<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .map(
                <type_sitter_lib::Either2<
                    Identifier<'tree>,
                    Metavariable<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .map(<Parameters<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        FunctionModifiers<'tree>,
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        FunctionModifiers<'tree>,
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionItem<'tree> {
    const KIND: &'static str = "function_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "function_modifiers", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionModifiers<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionModifiers<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    ExternModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, ExternModifier<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    ExternModifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionModifiers<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_modifiers" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionModifiers<'tree> {
    const KIND: &'static str = "function_modifiers";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "function_signature_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionSignatureItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionSignatureItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Identifier<'tree>, Metavariable<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .map(
                <type_sitter_lib::Either2<
                    Identifier<'tree>,
                    Metavariable<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .map(<Parameters<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        FunctionModifiers<'tree>,
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    FunctionModifiers<'tree>,
                    VisibilityModifier<'tree>,
                    WhereClause<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        FunctionModifiers<'tree>,
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionSignatureItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_signature_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionSignatureItem<'tree> {
    const KIND: &'static str = "function_signature_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "function_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FunctionType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FunctionType<'tree> {
    #[doc = concat!("Get the field `", "parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn parameters(&self) -> type_sitter_lib::NodeResult<'tree, Parameters<'tree>> {
        self.0
            .child_by_field_name("parameters")
            .map(<Parameters<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "return_type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn return_type(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0
            .child_by_field_name("return_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "trait", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either2<ScopedTypeIdentifier<'tree>, TypeIdentifier<'tree>>,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .map(
                <type_sitter_lib::Either2<
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<ForLifetimes<'tree>, FunctionModifiers<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        ForLifetimes<'tree>,
                        FunctionModifiers<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<ForLifetimes<'tree>, FunctionModifiers<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        ForLifetimes<'tree>,
                        FunctionModifiers<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FunctionType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "function_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FunctionType<'tree> {
    const KIND: &'static str = "function_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "generic_function", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericFunction<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericFunction<'tree> {
    #[doc = concat!("Get the field `", "function", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn function(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either3<
            FieldExpression<'tree>,
            Identifier<'tree>,
            ScopedIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("function")
            .map(
                <type_sitter_lib::Either3<
                    FieldExpression<'tree>,
                    Identifier<'tree>,
                    ScopedIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericFunction<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_function" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericFunction<'tree> {
    const KIND: &'static str = "generic_function";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "generic_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either3<
            ScopedIdentifier<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("type")
            .map(
                <type_sitter_lib::Either3<
                    ScopedIdentifier<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericType<'tree> {
    const KIND: &'static str = "generic_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "generic_type_with_turbofish", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GenericTypeWithTurbofish<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GenericTypeWithTurbofish<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<ScopedIdentifier<'tree>, TypeIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("type")
            .map(
                <type_sitter_lib::Either2<
                    ScopedIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GenericTypeWithTurbofish<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "generic_type_with_turbofish" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GenericTypeWithTurbofish<'tree> {
    const KIND: &'static str = "generic_type_with_turbofish";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "higher_ranked_trait_bound", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct HigherRankedTraitBound<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> HigherRankedTraitBound<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for HigherRankedTraitBound<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "higher_ranked_trait_bound" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for HigherRankedTraitBound<'tree> {
    const KIND: &'static str = "higher_ranked_trait_bound";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "if_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IfExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IfExpression<'tree> {
    #[doc = concat!("Get the field `", "alternative", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alternative(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, ElseClause<'tree>>> {
        self.0
            .child_by_field_name("alternative")
            .map(<ElseClause<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "condition", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either3<Expression<'tree>, LetChain<'tree>, LetCondition<'tree>>,
    > {
        self.0
            .child_by_field_name("condition")
            .map(
                <type_sitter_lib::Either3<
                    Expression<'tree>,
                    LetChain<'tree>,
                    LetCondition<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "consequence", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn consequence(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("consequence")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IfExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IfExpression<'tree> {
    const KIND: &'static str = "if_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "impl_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ImplItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ImplItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "trait", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#trait(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either3<
                GenericType<'tree>,
                ScopedTypeIdentifier<'tree>,
                TypeIdentifier<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("trait")
            .map(
                <type_sitter_lib::Either3<
                    GenericType<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, WhereClause<'tree>>> {
        self.0.named_child(0).map(<WhereClause<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ImplItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "impl_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ImplItem<'tree> {
    const KIND: &'static str = "impl_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "index_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IndexExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IndexExpression<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Expression<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Expression<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IndexExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "index_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IndexExpression<'tree> {
    const KIND: &'static str = "index_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "inner_attribute_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct InnerAttributeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> InnerAttributeItem<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Attribute<'tree>> {
        self.0
            .named_child(0)
            .map(<Attribute<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for InnerAttributeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "inner_attribute_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for InnerAttributeItem<'tree> {
    const KIND: &'static str = "inner_attribute_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "let_chain", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetChain<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetChain<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, LetCondition<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, LetCondition<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, LetCondition<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, LetCondition<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetChain<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_chain" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetChain<'tree> {
    const KIND: &'static str = "let_chain";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "let_condition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetCondition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetCondition<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetCondition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_condition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetCondition<'tree> {
    const KIND: &'static str = "let_condition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "let_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LetDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LetDeclaration<'tree> {
    #[doc = concat!("Get the field `", "alternative", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alternative(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Block<'tree>>> {
        self.0
            .child_by_field_name("alternative")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> Option<type_sitter_lib::NodeResult<'tree, Type<'tree>>> {
        self.0.child_by_field_name("type").map(<Type<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.named_child(0).map(<MutableSpecifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LetDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LetDeclaration<'tree> {
    const KIND: &'static str = "let_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "lifetime", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Lifetime<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Lifetime<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .named_child(0)
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lifetime<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "lifetime" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Lifetime<'tree> {
    const KIND: &'static str = "lifetime";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "loop_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LoopExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LoopExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.named_child(0).map(<LoopLabel<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopExpression<'tree> {
    const KIND: &'static str = "loop_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "loop_label", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LoopLabel<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LoopLabel<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .named_child(0)
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LoopLabel<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop_label" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LoopLabel<'tree> {
    const KIND: &'static str = "loop_label";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "macro_definition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroDefinition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroDefinition<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, MacroRule<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    MacroRule<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, MacroRule<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    MacroRule<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroDefinition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_definition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroDefinition<'tree> {
    const KIND: &'static str = "macro_definition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "macro_invocation", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroInvocation<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroInvocation<'tree> {
    #[doc = concat!("Get the field `", "macro", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#macro(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Identifier<'tree>, ScopedIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("macro")
            .map(
                <type_sitter_lib::Either2<
                    Identifier<'tree>,
                    ScopedIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, TokenTree<'tree>> {
        self.0
            .named_child(0)
            .map(<TokenTree<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroInvocation<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_invocation" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroInvocation<'tree> {
    const KIND: &'static str = "macro_invocation";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "macro_rule", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MacroRule<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MacroRule<'tree> {
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(&self) -> type_sitter_lib::NodeResult<'tree, TokenTreePattern<'tree>> {
        self.0
            .child_by_field_name("left")
            .map(<TokenTreePattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "right", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn right(&self) -> type_sitter_lib::NodeResult<'tree, TokenTree<'tree>> {
        self.0
            .child_by_field_name("right")
            .map(<TokenTree<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MacroRule<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_rule" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MacroRule<'tree> {
    const KIND: &'static str = "macro_rule";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "match_arm", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchArm<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchArm<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(&self) -> type_sitter_lib::NodeResult<'tree, MatchPattern<'tree>> {
        self.0
            .child_by_field_name("pattern")
            .map(<MatchPattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, AttributeItem<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, AttributeItem<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    AttributeItem<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchArm<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_arm" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchArm<'tree> {
    const KIND: &'static str = "match_arm";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "match_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchBlock<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    MatchArm<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, MatchArm<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    MatchArm<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchBlock<'tree> {
    const KIND: &'static str = "match_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "match_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, MatchBlock<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<MatchBlock<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchExpression<'tree> {
    const KIND: &'static str = "match_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "match_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MatchPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MatchPattern<'tree> {
    #[doc = concat!("Get the field `", "condition", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either3<
                Expression<'tree>,
                LetChain<'tree>,
                LetCondition<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("condition")
            .map(
                <type_sitter_lib::Either3<
                    Expression<'tree>,
                    LetChain<'tree>,
                    LetCondition<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .named_child(0)
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MatchPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MatchPattern<'tree> {
    const KIND: &'static str = "match_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "mod_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ModItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ModItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ModItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mod_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ModItem<'tree> {
    const KIND: &'static str = "mod_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "mut_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MutPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MutPattern<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mut_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MutPattern<'tree> {
    const KIND: &'static str = "mut_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "negative_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct NegativeLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> NegativeLiteral<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<FloatLiteral<'tree>, IntegerLiteral<'tree>>,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either2<
                    FloatLiteral<'tree>,
                    IntegerLiteral<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for NegativeLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "negative_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for NegativeLiteral<'tree> {
    const KIND: &'static str = "negative_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "optional_type_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OptionalTypeParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OptionalTypeParameter<'tree> {
    #[doc = concat!("Get the field `", "default_type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn default_type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("default_type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<ConstrainedTypeParameter<'tree>, TypeIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("name")
            .map(
                <type_sitter_lib::Either2<
                    ConstrainedTypeParameter<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OptionalTypeParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "optional_type_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OptionalTypeParameter<'tree> {
    const KIND: &'static str = "optional_type_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "or_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrPattern<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "or_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrPattern<'tree> {
    const KIND: &'static str = "or_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "ordered_field_declaration_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrderedFieldDeclarationList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrderedFieldDeclarationList<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn types<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Type<'tree>>,
        >,
    > + 'a {
        self.0
            .children_by_field_name("type", c)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, VisibilityModifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        AttributeItem<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, VisibilityModifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        AttributeItem<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrderedFieldDeclarationList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ordered_field_declaration_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrderedFieldDeclarationList<'tree> {
    const KIND: &'static str = "ordered_field_declaration_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Parameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Parameter<'tree> {
    #[doc = concat!("Get the field `", "pattern", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn pattern(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Pattern<'tree>, _Self<'tree>>,
    > {
        self.0
            .child_by_field_name("pattern")
            .map(
                <type_sitter_lib::Either2<
                    Pattern<'tree>,
                    _Self<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.named_child(0).map(<MutableSpecifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Parameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Parameter<'tree> {
    const KIND: &'static str = "parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Parameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Parameters<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either5<
                    Type<'tree>,
                    AttributeItem<'tree>,
                    Parameter<'tree>,
                    SelfParameter<'tree>,
                    VariadicParameter<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either5<
                        Type<'tree>,
                        AttributeItem<'tree>,
                        Parameter<'tree>,
                        SelfParameter<'tree>,
                        VariadicParameter<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either5<
                    Type<'tree>,
                    AttributeItem<'tree>,
                    Parameter<'tree>,
                    SelfParameter<'tree>,
                    VariadicParameter<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either5<
                        Type<'tree>,
                        AttributeItem<'tree>,
                        Parameter<'tree>,
                        SelfParameter<'tree>,
                        VariadicParameter<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Parameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Parameters<'tree> {
    const KIND: &'static str = "parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "parenthesized_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ParenthesizedExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ParenthesizedExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ParenthesizedExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "parenthesized_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ParenthesizedExpression<'tree> {
    const KIND: &'static str = "parenthesized_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "pointer_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PointerType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PointerType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.named_child(0).map(<MutableSpecifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PointerType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pointer_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PointerType<'tree> {
    const KIND: &'static str = "pointer_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "qualified_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct QualifiedType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> QualifiedType<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("alias")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for QualifiedType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "qualified_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for QualifiedType<'tree> {
    const KIND: &'static str = "qualified_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "range_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RangeExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RangeExpression<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Expression<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Expression<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    Expression<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RangeExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RangeExpression<'tree> {
    const KIND: &'static str = "range_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "range_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RangePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RangePattern<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either7<
                    LiteralPattern<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either7<
                        LiteralPattern<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        ScopedIdentifier<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either7<
                    LiteralPattern<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either7<
                        LiteralPattern<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        ScopedIdentifier<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RangePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "range_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RangePattern<'tree> {
    const KIND: &'static str = "range_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "ref_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RefPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RefPattern<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Pattern<'tree>> {
        self.0
            .named_child(0)
            .map(<Pattern<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RefPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ref_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RefPattern<'tree> {
    const KIND: &'static str = "ref_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "reference_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferenceExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferenceExpression<'tree> {
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, MutableSpecifier<'tree>>> {
        self.0.named_child(0).map(<MutableSpecifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferenceExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferenceExpression<'tree> {
    const KIND: &'static str = "reference_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "reference_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferencePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferencePattern<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Pattern<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferencePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferencePattern<'tree> {
    const KIND: &'static str = "reference_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "reference_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReferenceType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReferenceType<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Lifetime<'tree>, MutableSpecifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReferenceType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "reference_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReferenceType<'tree> {
    const KIND: &'static str = "reference_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "remaining_field_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RemainingFieldPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RemainingFieldPattern<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RemainingFieldPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "remaining_field_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RemainingFieldPattern<'tree> {
    const KIND: &'static str = "remaining_field_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "removed_trait_bound", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RemovedTraitBound<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RemovedTraitBound<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .named_child(0)
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RemovedTraitBound<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "removed_trait_bound" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RemovedTraitBound<'tree> {
    const KIND: &'static str = "removed_trait_bound";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "return_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ReturnExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ReturnExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.named_child(0).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ReturnExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ReturnExpression<'tree> {
    const KIND: &'static str = "return_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "scoped_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedIdentifier<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either8<
                BracketedType<'tree>,
                Crate<'tree>,
                GenericType<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("path")
            .map(
                <type_sitter_lib::Either8<
                    BracketedType<'tree>,
                    Crate<'tree>,
                    GenericType<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedIdentifier<'tree> {
    const KIND: &'static str = "scoped_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "scoped_type_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedTypeIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedTypeIdentifier<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either8<
                BracketedType<'tree>,
                Crate<'tree>,
                GenericType<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("path")
            .map(
                <type_sitter_lib::Either8<
                    BracketedType<'tree>,
                    Crate<'tree>,
                    GenericType<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedTypeIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_type_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedTypeIdentifier<'tree> {
    const KIND: &'static str = "scoped_type_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "scoped_use_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ScopedUseList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ScopedUseList<'tree> {
    #[doc = concat!("Get the field `", "list", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn list(&self) -> type_sitter_lib::NodeResult<'tree, UseList<'tree>> {
        self.0
            .child_by_field_name("list")
            .map(<UseList<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("path")
            .map(
                <type_sitter_lib::Either6<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ScopedUseList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "scoped_use_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ScopedUseList<'tree> {
    const KIND: &'static str = "scoped_use_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "self_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SelfParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SelfParameter<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                    _Self<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        Lifetime<'tree>,
                        MutableSpecifier<'tree>,
                        _Self<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either3<
                    Lifetime<'tree>,
                    MutableSpecifier<'tree>,
                    _Self<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either3<
                        Lifetime<'tree>,
                        MutableSpecifier<'tree>,
                        _Self<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SelfParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SelfParameter<'tree> {
    const KIND: &'static str = "self_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "shorthand_field_initializer", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ShorthandFieldInitializer<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ShorthandFieldInitializer<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, Identifier<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<AttributeItem<'tree>, Identifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<AttributeItem<'tree>, Identifier<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<AttributeItem<'tree>, Identifier<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ShorthandFieldInitializer<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_initializer" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ShorthandFieldInitializer<'tree> {
    const KIND: &'static str = "shorthand_field_initializer";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "slice_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SlicePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SlicePattern<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SlicePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "slice_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SlicePattern<'tree> {
    const KIND: &'static str = "slice_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "source_file", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SourceFile<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SourceFile<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    DeclarationStatement<'tree>,
                    ExpressionStatement<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        DeclarationStatement<'tree>,
                        ExpressionStatement<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    DeclarationStatement<'tree>,
                    ExpressionStatement<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        DeclarationStatement<'tree>,
                        ExpressionStatement<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SourceFile<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "source_file" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SourceFile<'tree> {
    const KIND: &'static str = "source_file";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "static_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StaticItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StaticItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    MutableSpecifier<'tree>,
                    VisibilityModifier<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        MutableSpecifier<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    MutableSpecifier<'tree>,
                    VisibilityModifier<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        MutableSpecifier<'tree>,
                        VisibilityModifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StaticItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "static_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StaticItem<'tree> {
    const KIND: &'static str = "static_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "string_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StringLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StringLiteral<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    EscapeSequence<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, EscapeSequence<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    EscapeSequence<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StringLiteral<'tree> {
    const KIND: &'static str = "string_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "struct_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, FieldInitializerList<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<FieldInitializerList<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either3<
            GenericTypeWithTurbofish<'tree>,
            ScopedTypeIdentifier<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("name")
            .map(
                <type_sitter_lib::Either3<
                    GenericTypeWithTurbofish<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructExpression<'tree> {
    const KIND: &'static str = "struct_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "struct_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either2<
                FieldDeclarationList<'tree>,
                OrderedFieldDeclarationList<'tree>,
            >,
        >,
    > {
        self.0
            .child_by_field_name("body")
            .map(
                <type_sitter_lib::Either2<
                    FieldDeclarationList<'tree>,
                    OrderedFieldDeclarationList<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructItem<'tree> {
    const KIND: &'static str = "struct_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "struct_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct StructPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> StructPattern<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<ScopedTypeIdentifier<'tree>, TypeIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("type")
            .map(
                <type_sitter_lib::Either2<
                    ScopedTypeIdentifier<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    FieldPattern<'tree>,
                    RemainingFieldPattern<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        FieldPattern<'tree>,
                        RemainingFieldPattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<
                    FieldPattern<'tree>,
                    RemainingFieldPattern<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        FieldPattern<'tree>,
                        RemainingFieldPattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for StructPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for StructPattern<'tree> {
    const KIND: &'static str = "struct_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "token_binding_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenBindingPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenBindingPattern<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, Metavariable<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<Metavariable<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, FragmentSpecifier<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<FragmentSpecifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenBindingPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_binding_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenBindingPattern<'tree> {
    const KIND: &'static str = "token_binding_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "token_repetition", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenRepetition<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenRepetition<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenRepetition<'tree>,
                        TokenTree<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenRepetition<'tree>,
                        TokenTree<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenRepetition<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenRepetition<'tree> {
    const KIND: &'static str = "token_repetition";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "token_repetition_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenRepetitionPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenRepetitionPattern<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either11<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenBindingPattern<'tree>,
                        TokenRepetitionPattern<'tree>,
                        TokenTreePattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either11<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenBindingPattern<'tree>,
                        TokenRepetitionPattern<'tree>,
                        TokenTreePattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenRepetitionPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_repetition_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenRepetitionPattern<'tree> {
    const KIND: &'static str = "token_repetition_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "token_tree", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenTree<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenTree<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenRepetition<'tree>,
                        TokenTree<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenRepetition<'tree>,
                    TokenTree<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenRepetition<'tree>,
                        TokenTree<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenTree<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenTree<'tree> {
    const KIND: &'static str = "token_tree";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "token_tree_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TokenTreePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TokenTreePattern<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either11<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenBindingPattern<'tree>,
                        TokenRepetitionPattern<'tree>,
                        TokenTreePattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either11<
                    Literal<'tree>,
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    MutableSpecifier<'tree>,
                    PrimitiveType<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    TokenBindingPattern<'tree>,
                    TokenRepetitionPattern<'tree>,
                    TokenTreePattern<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either11<
                        Literal<'tree>,
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        MutableSpecifier<'tree>,
                        PrimitiveType<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        TokenBindingPattern<'tree>,
                        TokenRepetitionPattern<'tree>,
                        TokenTreePattern<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TokenTreePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "token_tree_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TokenTreePattern<'tree> {
    const KIND: &'static str = "token_tree_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "trait_bounds", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TraitBounds<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TraitBounds<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either4<
                    Type<'tree>,
                    HigherRankedTraitBound<'tree>,
                    Lifetime<'tree>,
                    RemovedTraitBound<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either4<
                        Type<'tree>,
                        HigherRankedTraitBound<'tree>,
                        Lifetime<'tree>,
                        RemovedTraitBound<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either4<
                    Type<'tree>,
                    HigherRankedTraitBound<'tree>,
                    Lifetime<'tree>,
                    RemovedTraitBound<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either4<
                        Type<'tree>,
                        HigherRankedTraitBound<'tree>,
                        Lifetime<'tree>,
                        RemovedTraitBound<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TraitBounds<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_bounds" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TraitBounds<'tree> {
    const KIND: &'static str = "trait_bounds";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "trait_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TraitItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TraitItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, DeclarationList<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<DeclarationList<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "bounds", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TraitItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TraitItem<'tree> {
    const KIND: &'static str = "trait_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "try_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TryExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "try_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TryExpression<'tree> {
    const KIND: &'static str = "try_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "tuple_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleExpression<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<Expression<'tree>, AttributeItem<'tree>>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleExpression<'tree> {
    const KIND: &'static str = "tuple_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "tuple_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TuplePattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TuplePattern<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TuplePattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TuplePattern<'tree> {
    const KIND: &'static str = "tuple_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "tuple_struct_pattern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleStructPattern<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleStructPattern<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either2<Identifier<'tree>, ScopedIdentifier<'tree>>,
    > {
        self.0
            .child_by_field_name("type")
            .map(
                <type_sitter_lib::Either2<
                    Identifier<'tree>,
                    ScopedIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Pattern<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<'tree, Pattern<'tree>> as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleStructPattern<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_struct_pattern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleStructPattern<'tree> {
    const KIND: &'static str = "tuple_struct_pattern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "tuple_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TupleType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TupleType<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, Type<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<'tree, type_sitter_lib::ExtraOr<'tree, Type<'tree>>>,
    > {
        self.0
            .named_child(i)
            .map(<type_sitter_lib::ExtraOr<'tree, Type<'tree>> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TupleType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tuple_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TupleType<'tree> {
    const KIND: &'static str = "tuple_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_arguments", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeArguments<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeArguments<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either5<
                    Literal<'tree>,
                    Type<'tree>,
                    Block<'tree>,
                    Lifetime<'tree>,
                    TypeBinding<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either5<
                        Literal<'tree>,
                        Type<'tree>,
                        Block<'tree>,
                        Lifetime<'tree>,
                        TypeBinding<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either5<
                    Literal<'tree>,
                    Type<'tree>,
                    Block<'tree>,
                    Lifetime<'tree>,
                    TypeBinding<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either5<
                        Literal<'tree>,
                        Type<'tree>,
                        Block<'tree>,
                        Lifetime<'tree>,
                        TypeBinding<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeArguments<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_arguments" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeArguments<'tree> {
    const KIND: &'static str = "type_arguments";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_binding", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeBinding<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeBinding<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_arguments", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_arguments(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeArguments<'tree>>> {
        self.0
            .child_by_field_name("type_arguments")
            .map(<TypeArguments<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeBinding<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_binding" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeBinding<'tree> {
    const KIND: &'static str = "type_binding";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_cast_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeCastExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeCastExpression<'tree> {
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "value", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn value(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .child_by_field_name("value")
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeCastExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_cast_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeCastExpression<'tree> {
    const KIND: &'static str = "type_cast_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeItem<'tree> {
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn r#type(&self) -> type_sitter_lib::NodeResult<'tree, Type<'tree>> {
        self.0
            .child_by_field_name("type")
            .map(<Type<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeItem<'tree> {
    const KIND: &'static str = "type_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_parameters", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeParameters<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeParameters<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either6<
                    ConstParameter<'tree>,
                    ConstrainedTypeParameter<'tree>,
                    Lifetime<'tree>,
                    Metavariable<'tree>,
                    OptionalTypeParameter<'tree>,
                    TypeIdentifier<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either6<
                        ConstParameter<'tree>,
                        ConstrainedTypeParameter<'tree>,
                        Lifetime<'tree>,
                        Metavariable<'tree>,
                        OptionalTypeParameter<'tree>,
                        TypeIdentifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either6<
                    ConstParameter<'tree>,
                    ConstrainedTypeParameter<'tree>,
                    Lifetime<'tree>,
                    Metavariable<'tree>,
                    OptionalTypeParameter<'tree>,
                    TypeIdentifier<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either6<
                        ConstParameter<'tree>,
                        ConstrainedTypeParameter<'tree>,
                        Lifetime<'tree>,
                        Metavariable<'tree>,
                        OptionalTypeParameter<'tree>,
                        TypeIdentifier<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeParameters<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_parameters" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeParameters<'tree> {
    const KIND: &'static str = "type_parameters";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "unary_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnaryExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnaryExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Expression<'tree>> {
        self.0
            .named_child(0)
            .map(<Expression<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnaryExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unary_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnaryExpression<'tree> {
    const KIND: &'static str = "unary_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "union_item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnionItem<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnionItem<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(
        &self,
    ) -> type_sitter_lib::NodeResult<'tree, FieldDeclarationList<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<FieldDeclarationList<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "name", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn name(&self) -> type_sitter_lib::NodeResult<'tree, TypeIdentifier<'tree>> {
        self.0
            .child_by_field_name("name")
            .map(<TypeIdentifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "type_parameters", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn type_parameters(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, TypeParameters<'tree>>> {
        self.0
            .child_by_field_name("type_parameters")
            .map(<TypeParameters<'tree> as TryFrom<_>>::try_from)
    }
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either2<VisibilityModifier<'tree>, WhereClause<'tree>>,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either2<
                        VisibilityModifier<'tree>,
                        WhereClause<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnionItem<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "union_item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnionItem<'tree> {
    const KIND: &'static str = "union_item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "unit_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnitExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnitExpression<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnitExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnitExpression<'tree> {
    const KIND: &'static str = "unit_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "unit_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnitType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnitType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnitType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unit_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnitType<'tree> {
    const KIND: &'static str = "unit_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "unsafe_block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UnsafeBlock<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UnsafeBlock<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .named_child(0)
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UnsafeBlock<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unsafe_block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UnsafeBlock<'tree> {
    const KIND: &'static str = "unsafe_block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "use_as_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseAsClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseAsClause<'tree> {
    #[doc = concat!("Get the field `", "alias", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn alias(&self) -> type_sitter_lib::NodeResult<'tree, Identifier<'tree>> {
        self.0
            .child_by_field_name("alias")
            .map(<Identifier<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "path", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn path(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either6<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            _Self<'tree>,
            _Super<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("path")
            .map(
                <type_sitter_lib::Either6<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseAsClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_as_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseAsClause<'tree> {
    const KIND: &'static str = "use_as_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "use_declaration", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseDeclaration<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseDeclaration<'tree> {
    #[doc = concat!("Get the field `", "argument", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn argument(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either10<
            Crate<'tree>,
            Identifier<'tree>,
            Metavariable<'tree>,
            ScopedIdentifier<'tree>,
            ScopedUseList<'tree>,
            _Self<'tree>,
            _Super<'tree>,
            UseAsClause<'tree>,
            UseList<'tree>,
            UseWildcard<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("argument")
            .map(
                <type_sitter_lib::Either10<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    ScopedUseList<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    UseAsClause<'tree>,
                    UseList<'tree>,
                    UseWildcard<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, VisibilityModifier<'tree>>> {
        self.0.named_child(0).map(<VisibilityModifier<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseDeclaration<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_declaration" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseDeclaration<'tree> {
    const KIND: &'static str = "use_declaration";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "use_list", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseList<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseList<'tree> {
    ///Get the node's named children
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    ScopedUseList<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    UseAsClause<'tree>,
                    UseList<'tree>,
                    UseWildcard<'tree>,
                >,
            >,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        ScopedIdentifier<'tree>,
                        ScopedUseList<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        UseAsClause<'tree>,
                        UseList<'tree>,
                        UseWildcard<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<
                'tree,
                type_sitter_lib::Either10<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    ScopedUseList<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                    UseAsClause<'tree>,
                    UseList<'tree>,
                    UseWildcard<'tree>,
                >,
            >,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    type_sitter_lib::Either10<
                        Crate<'tree>,
                        Identifier<'tree>,
                        Metavariable<'tree>,
                        ScopedIdentifier<'tree>,
                        ScopedUseList<'tree>,
                        _Self<'tree>,
                        _Super<'tree>,
                        UseAsClause<'tree>,
                        UseList<'tree>,
                        UseWildcard<'tree>,
                    >,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseList<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_list" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseList<'tree> {
    const KIND: &'static str = "use_list";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "use_wildcard", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UseWildcard<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UseWildcard<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either6<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UseWildcard<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use_wildcard" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UseWildcard<'tree> {
    const KIND: &'static str = "use_wildcard";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "variadic_parameter", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VariadicParameter<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VariadicParameter<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VariadicParameter<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "variadic_parameter" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VariadicParameter<'tree> {
    const KIND: &'static str = "variadic_parameter";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "visibility_modifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VisibilityModifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VisibilityModifier<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::Either6<
                Crate<'tree>,
                Identifier<'tree>,
                Metavariable<'tree>,
                ScopedIdentifier<'tree>,
                _Self<'tree>,
                _Super<'tree>,
            >,
        >,
    > {
        self.0
            .named_child(0)
            .map(
                <type_sitter_lib::Either6<
                    Crate<'tree>,
                    Identifier<'tree>,
                    Metavariable<'tree>,
                    ScopedIdentifier<'tree>,
                    _Self<'tree>,
                    _Super<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VisibilityModifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "visibility_modifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VisibilityModifier<'tree> {
    const KIND: &'static str = "visibility_modifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "where_clause", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WhereClause<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WhereClause<'tree> {
    ///Get the node's named children
    ///This is guaranteed to return at least one child
    #[allow(dead_code)]
    #[inline]
    pub fn children<'a>(
        &'a self,
        c: &'a mut tree_sitter::TreeCursor<'tree>,
    ) -> impl Iterator<
        Item = type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>>,
        >,
    > + 'a {
        self.0
            .named_children(c)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    WherePredicate<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
    ///Get the node's named child #i
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
        i: usize,
    ) -> Option<
        type_sitter_lib::NodeResult<
            'tree,
            type_sitter_lib::ExtraOr<'tree, WherePredicate<'tree>>,
        >,
    > {
        self.0
            .named_child(i)
            .map(
                <type_sitter_lib::ExtraOr<
                    'tree,
                    WherePredicate<'tree>,
                > as TryFrom<_>>::try_from,
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WhereClause<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_clause" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WhereClause<'tree> {
    const KIND: &'static str = "where_clause";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "where_predicate", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WherePredicate<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WherePredicate<'tree> {
    #[doc = concat!("Get the field `", "bounds", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn bounds(&self) -> type_sitter_lib::NodeResult<'tree, TraitBounds<'tree>> {
        self.0
            .child_by_field_name("bounds")
            .map(<TraitBounds<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "left", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn left(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either10<
            ArrayType<'tree>,
            GenericType<'tree>,
            HigherRankedTraitBound<'tree>,
            Lifetime<'tree>,
            PointerType<'tree>,
            PrimitiveType<'tree>,
            ReferenceType<'tree>,
            ScopedTypeIdentifier<'tree>,
            TupleType<'tree>,
            TypeIdentifier<'tree>,
        >,
    > {
        self.0
            .child_by_field_name("left")
            .map(
                <type_sitter_lib::Either10<
                    ArrayType<'tree>,
                    GenericType<'tree>,
                    HigherRankedTraitBound<'tree>,
                    Lifetime<'tree>,
                    PointerType<'tree>,
                    PrimitiveType<'tree>,
                    ReferenceType<'tree>,
                    ScopedTypeIdentifier<'tree>,
                    TupleType<'tree>,
                    TypeIdentifier<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WherePredicate<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where_predicate" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WherePredicate<'tree> {
    const KIND: &'static str = "where_predicate";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "while_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WhileExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WhileExpression<'tree> {
    #[doc = concat!("Get the field `", "body", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn body(&self) -> type_sitter_lib::NodeResult<'tree, Block<'tree>> {
        self.0
            .child_by_field_name("body")
            .map(<Block<'tree> as TryFrom<_>>::try_from)
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    #[doc = concat!("Get the field `", "condition", "`")]
    #[allow(dead_code)]
    #[inline]
    pub fn condition(
        &self,
    ) -> type_sitter_lib::NodeResult<
        'tree,
        type_sitter_lib::Either3<Expression<'tree>, LetChain<'tree>, LetCondition<'tree>>,
    > {
        self.0
            .child_by_field_name("condition")
            .map(
                <type_sitter_lib::Either3<
                    Expression<'tree>,
                    LetChain<'tree>,
                    LetCondition<'tree>,
                > as TryFrom<_>>::try_from,
            )
            .expect(
                "tree-sitter node missing its required child, there should at least be a MISSING node in its place",
            )
    }
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(&self) -> Option<type_sitter_lib::NodeResult<'tree, LoopLabel<'tree>>> {
        self.0.named_child(0).map(<LoopLabel<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WhileExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "while_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WhileExpression<'tree> {
    const KIND: &'static str = "while_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "yield_expression", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct YieldExpression<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> YieldExpression<'tree> {
    ///Get the node's only named child
    #[allow(dead_code)]
    #[inline]
    pub fn child(
        &self,
    ) -> Option<type_sitter_lib::NodeResult<'tree, Expression<'tree>>> {
        self.0.named_child(0).map(<Expression<'tree> as TryFrom<_>>::try_from)
    }
}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for YieldExpression<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "yield_expression" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for YieldExpression<'tree> {
    const KIND: &'static str = "yield_expression";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "!", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Not<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Not<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Not<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "!" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Not<'tree> {
    const KIND: &'static str = "!";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "!=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct NotEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> NotEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for NotEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "!=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for NotEq<'tree> {
    const KIND: &'static str = "!=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "\"", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DoubleQuote<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DoubleQuote<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DoubleQuote<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "\"" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DoubleQuote<'tree> {
    const KIND: &'static str = "\"";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "#", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Hash<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Hash<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Hash<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "#" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Hash<'tree> {
    const KIND: &'static str = "#";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "$", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Dollar<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Dollar<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dollar<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "$" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Dollar<'tree> {
    const KIND: &'static str = "$";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "%", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Mod<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Mod<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mod<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "%" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Mod<'tree> {
    const KIND: &'static str = "%";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "%=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ModEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ModEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ModEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "%=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ModEq<'tree> {
    const KIND: &'static str = "%=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "&", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct And<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> And<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for And<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for And<'tree> {
    const KIND: &'static str = "&";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "&&", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AndAnd<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AndAnd<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AndAnd<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&&" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AndAnd<'tree> {
    const KIND: &'static str = "&&";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "&=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AndEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AndEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AndEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "&=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AndEq<'tree> {
    const KIND: &'static str = "&=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "'", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Quote<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Quote<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Quote<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "'" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Quote<'tree> {
    const KIND: &'static str = "'";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "(", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LParen<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LParen<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LParen<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "(" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LParen<'tree> {
    const KIND: &'static str = "(";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ")", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RParen<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RParen<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RParen<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ")" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RParen<'tree> {
    const KIND: &'static str = ")";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "*", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Mul<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Mul<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Mul<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "*" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Mul<'tree> {
    const KIND: &'static str = "*";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "*=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MulEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MulEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MulEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "*=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MulEq<'tree> {
    const KIND: &'static str = "*=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "+", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Add<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Add<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Add<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "+" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Add<'tree> {
    const KIND: &'static str = "+";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "+=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AddEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AddEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AddEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "+=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AddEq<'tree> {
    const KIND: &'static str = "+=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ",", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Comma<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Comma<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Comma<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "," {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Comma<'tree> {
    const KIND: &'static str = ",";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "-", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Sub<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Sub<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Sub<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "-" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Sub<'tree> {
    const KIND: &'static str = "-";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "-=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SubEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SubEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SubEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "-=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SubEq<'tree> {
    const KIND: &'static str = "-=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "->", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct SubGt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> SubGt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for SubGt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "->" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for SubGt<'tree> {
    const KIND: &'static str = "->";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ".", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Dot<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Dot<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Dot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "." {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Dot<'tree> {
    const KIND: &'static str = ".";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "..", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DotDot<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DotDot<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ".." {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDot<'tree> {
    const KIND: &'static str = "..";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "...", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DotDotDot<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DotDotDot<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDotDot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "..." {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDotDot<'tree> {
    const KIND: &'static str = "...";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "..=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DotDotEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DotDotEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DotDotEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "..=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DotDotEq<'tree> {
    const KIND: &'static str = "..=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "/", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Div<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Div<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Div<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "/" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Div<'tree> {
    const KIND: &'static str = "/";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "/=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DivEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DivEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DivEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "/=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DivEq<'tree> {
    const KIND: &'static str = "/=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ":", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Colon<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Colon<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Colon<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ":" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Colon<'tree> {
    const KIND: &'static str = ":";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "::", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ColonColon<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ColonColon<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ColonColon<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "::" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ColonColon<'tree> {
    const KIND: &'static str = "::";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ";", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Semicolon<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Semicolon<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Semicolon<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ";" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Semicolon<'tree> {
    const KIND: &'static str = ";";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "<", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Lt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Lt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Lt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Lt<'tree> {
    const KIND: &'static str = "<";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "<<", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LtLt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LtLt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtLt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<<" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLt<'tree> {
    const KIND: &'static str = "<<";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "<<=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LtLtEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LtLtEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtLtEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<<=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LtLtEq<'tree> {
    const KIND: &'static str = "<<=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "<=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LtEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LtEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LtEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "<=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LtEq<'tree> {
    const KIND: &'static str = "<=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Eq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Eq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Eq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Eq<'tree> {
    const KIND: &'static str = "=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "==", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EqEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EqEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EqEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "==" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EqEq<'tree> {
    const KIND: &'static str = "==";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "=>", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EqGt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EqGt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EqGt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "=>" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EqGt<'tree> {
    const KIND: &'static str = "=>";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ">", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Gt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Gt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Gt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Gt<'tree> {
    const KIND: &'static str = ">";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ">=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GtEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GtEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GtEq<'tree> {
    const KIND: &'static str = ">=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ">>", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GtGt<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GtGt<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtGt<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">>" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGt<'tree> {
    const KIND: &'static str = ">>";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", ">>=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct GtGtEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> GtGtEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for GtGtEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == ">>=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for GtGtEq<'tree> {
    const KIND: &'static str = ">>=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "?", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Question<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Question<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Question<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "?" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Question<'tree> {
    const KIND: &'static str = "?";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "@", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct At<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> At<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for At<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "@" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for At<'tree> {
    const KIND: &'static str = "@";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "[", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LBracket<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LBracket<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBracket<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "[" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LBracket<'tree> {
    const KIND: &'static str = "[";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "]", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RBracket<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RBracket<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBracket<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "]" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RBracket<'tree> {
    const KIND: &'static str = "]";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "^", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BitXor<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BitXor<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitXor<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "^" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXor<'tree> {
    const KIND: &'static str = "^";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "^=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BitXorEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BitXorEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BitXorEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "^=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BitXorEq<'tree> {
    const KIND: &'static str = "^=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "_", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct __<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> __<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for __<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "_" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for __<'tree> {
    const KIND: &'static str = "_";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "as", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AS<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AS<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AS<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "as" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AS<'tree> {
    const KIND: &'static str = "as";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "async", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ASYNC<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ASYNC<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ASYNC<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "async" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ASYNC<'tree> {
    const KIND: &'static str = "async";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "await", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct AWAIT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> AWAIT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for AWAIT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "await" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for AWAIT<'tree> {
    const KIND: &'static str = "await";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "block", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BLOCK<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BLOCK<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BLOCK<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BLOCK<'tree> {
    const KIND: &'static str = "block";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "block_comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BlockComment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BlockComment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BlockComment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "block_comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BlockComment<'tree> {
    const KIND: &'static str = "block_comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "break", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct BREAK<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> BREAK<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for BREAK<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "break" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for BREAK<'tree> {
    const KIND: &'static str = "break";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "char_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CharLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CharLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CharLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "char_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CharLiteral<'tree> {
    const KIND: &'static str = "char_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "const", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CONST<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CONST<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CONST<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "const" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CONST<'tree> {
    const KIND: &'static str = "const";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "continue", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct CONTINUE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> CONTINUE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for CONTINUE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "continue" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for CONTINUE<'tree> {
    const KIND: &'static str = "continue";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "crate", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Crate<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Crate<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Crate<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "crate" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Crate<'tree> {
    const KIND: &'static str = "crate";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "default", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DEFAULT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DEFAULT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DEFAULT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "default" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DEFAULT<'tree> {
    const KIND: &'static str = "default";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "dyn", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct DYN<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> DYN<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for DYN<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "dyn" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for DYN<'tree> {
    const KIND: &'static str = "dyn";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "else", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ELSE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ELSE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ELSE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "else" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ELSE<'tree> {
    const KIND: &'static str = "else";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "enum", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ENUM<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ENUM<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ENUM<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "enum" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ENUM<'tree> {
    const KIND: &'static str = "enum";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "escape_sequence", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EscapeSequence<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EscapeSequence<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EscapeSequence<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "escape_sequence" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EscapeSequence<'tree> {
    const KIND: &'static str = "escape_sequence";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "expr", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EXPR<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EXPR<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EXPR<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "expr" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EXPR<'tree> {
    const KIND: &'static str = "expr";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "extern", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct EXTERN<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> EXTERN<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for EXTERN<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "extern" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for EXTERN<'tree> {
    const KIND: &'static str = "extern";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "false", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FALSE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FALSE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FALSE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "false" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FALSE<'tree> {
    const KIND: &'static str = "false";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "field_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FieldIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FieldIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FieldIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "field_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FieldIdentifier<'tree> {
    const KIND: &'static str = "field_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "float_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FloatLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FloatLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FloatLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "float_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FloatLiteral<'tree> {
    const KIND: &'static str = "float_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "fn", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FN<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FN<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FN<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "fn" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FN<'tree> {
    const KIND: &'static str = "fn";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "for", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct FOR<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> FOR<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for FOR<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "for" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for FOR<'tree> {
    const KIND: &'static str = "for";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "ident", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IDENT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IDENT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IDENT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ident" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IDENT<'tree> {
    const KIND: &'static str = "ident";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Identifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Identifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Identifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Identifier<'tree> {
    const KIND: &'static str = "identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "if", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IF<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IF<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IF<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "if" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IF<'tree> {
    const KIND: &'static str = "if";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "impl", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IMPL<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IMPL<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IMPL<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "impl" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IMPL<'tree> {
    const KIND: &'static str = "impl";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "in", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IN<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IN<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IN<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "in" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IN<'tree> {
    const KIND: &'static str = "in";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "integer_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct IntegerLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> IntegerLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for IntegerLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "integer_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for IntegerLiteral<'tree> {
    const KIND: &'static str = "integer_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "item", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ITEM<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ITEM<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ITEM<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "item" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ITEM<'tree> {
    const KIND: &'static str = "item";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "let", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LET<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LET<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LET<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "let" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LET<'tree> {
    const KIND: &'static str = "let";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "lifetime", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LIFETIME<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LIFETIME<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LIFETIME<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "lifetime" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LIFETIME<'tree> {
    const KIND: &'static str = "lifetime";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "line_comment", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LineComment<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LineComment<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LineComment<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "line_comment" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LineComment<'tree> {
    const KIND: &'static str = "line_comment";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LITERAL<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LITERAL<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LITERAL<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LITERAL<'tree> {
    const KIND: &'static str = "literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "loop", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LOOP<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LOOP<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LOOP<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "loop" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LOOP<'tree> {
    const KIND: &'static str = "loop";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "macro_rules!", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MACROU5FRULESNot<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MACROU5FRULESNot<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MACROU5FRULESNot<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "macro_rules!" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MACROU5FRULESNot<'tree> {
    const KIND: &'static str = "macro_rules!";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "match", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MATCH<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MATCH<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MATCH<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "match" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MATCH<'tree> {
    const KIND: &'static str = "match";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "meta", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct META<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> META<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for META<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "meta" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for META<'tree> {
    const KIND: &'static str = "meta";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "metavariable", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Metavariable<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Metavariable<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Metavariable<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "metavariable" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Metavariable<'tree> {
    const KIND: &'static str = "metavariable";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "mod", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MOD<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MOD<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MOD<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mod" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MOD<'tree> {
    const KIND: &'static str = "mod";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "move", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MOVE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MOVE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MOVE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "move" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MOVE<'tree> {
    const KIND: &'static str = "move";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "mutable_specifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct MutableSpecifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> MutableSpecifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for MutableSpecifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "mutable_specifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for MutableSpecifier<'tree> {
    const KIND: &'static str = "mutable_specifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "pat", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PAT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PAT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PAT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pat" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PAT<'tree> {
    const KIND: &'static str = "pat";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "path", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PATH<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PATH<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PATH<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "path" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PATH<'tree> {
    const KIND: &'static str = "path";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "primitive_type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PrimitiveType<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PrimitiveType<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PrimitiveType<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "primitive_type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PrimitiveType<'tree> {
    const KIND: &'static str = "primitive_type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "pub", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct PUB<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> PUB<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for PUB<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "pub" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for PUB<'tree> {
    const KIND: &'static str = "pub";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "raw_string_literal", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RawStringLiteral<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RawStringLiteral<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RawStringLiteral<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "raw_string_literal" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RawStringLiteral<'tree> {
    const KIND: &'static str = "raw_string_literal";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "ref", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct REF<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> REF<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for REF<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ref" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for REF<'tree> {
    const KIND: &'static str = "ref";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "return", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RETURN<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RETURN<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RETURN<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "return" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RETURN<'tree> {
    const KIND: &'static str = "return";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "self", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct _Self<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> _Self<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for _Self<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "self" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for _Self<'tree> {
    const KIND: &'static str = "self";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "shorthand_field_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ShorthandFieldIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> ShorthandFieldIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for ShorthandFieldIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "shorthand_field_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for ShorthandFieldIdentifier<'tree> {
    const KIND: &'static str = "shorthand_field_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "static", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct STATIC<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> STATIC<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for STATIC<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "static" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for STATIC<'tree> {
    const KIND: &'static str = "static";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "stmt", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct STMT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> STMT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for STMT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "stmt" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for STMT<'tree> {
    const KIND: &'static str = "stmt";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "struct", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct STRUCT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> STRUCT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for STRUCT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "struct" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for STRUCT<'tree> {
    const KIND: &'static str = "struct";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "super", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct _Super<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> _Super<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for _Super<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "super" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for _Super<'tree> {
    const KIND: &'static str = "super";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "trait", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TRAIT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TRAIT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TRAIT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "trait" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TRAIT<'tree> {
    const KIND: &'static str = "trait";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "true", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TRUE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TRUE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TRUE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "true" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TRUE<'tree> {
    const KIND: &'static str = "true";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "tt", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TT<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TT<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TT<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "tt" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TT<'tree> {
    const KIND: &'static str = "tt";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "ty", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TY<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TY<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TY<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "ty" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TY<'tree> {
    const KIND: &'static str = "ty";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TYPE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TYPE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TYPE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TYPE<'tree> {
    const KIND: &'static str = "type";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "type_identifier", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct TypeIdentifier<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> TypeIdentifier<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for TypeIdentifier<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "type_identifier" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for TypeIdentifier<'tree> {
    const KIND: &'static str = "type_identifier";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "union", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UNION<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UNION<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UNION<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "union" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UNION<'tree> {
    const KIND: &'static str = "union";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "unsafe", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct UNSAFE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> UNSAFE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for UNSAFE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "unsafe" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for UNSAFE<'tree> {
    const KIND: &'static str = "unsafe";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "use", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct USE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> USE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for USE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "use" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for USE<'tree> {
    const KIND: &'static str = "use";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "vis", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct VIS<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> VIS<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for VIS<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "vis" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for VIS<'tree> {
    const KIND: &'static str = "vis";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "where", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WHERE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WHERE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WHERE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "where" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WHERE<'tree> {
    const KIND: &'static str = "where";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "while", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct WHILE<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> WHILE<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for WHILE<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "while" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for WHILE<'tree> {
    const KIND: &'static str = "while";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "yield", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct YIELD<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> YIELD<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for YIELD<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "yield" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for YIELD<'tree> {
    const KIND: &'static str = "yield";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "{", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct LBrace<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> LBrace<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for LBrace<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "{" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for LBrace<'tree> {
    const KIND: &'static str = "{";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "|", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct Or<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> Or<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for Or<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "|" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for Or<'tree> {
    const KIND: &'static str = "|";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "|=", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrEq<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrEq<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrEq<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "|=" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrEq<'tree> {
    const KIND: &'static str = "|=";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "||", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct OrOr<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> OrOr<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for OrOr<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "||" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for OrOr<'tree> {
    const KIND: &'static str = "||";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
#[doc = concat!("Typed node `", "}", "`")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct RBrace<'tree>(tree_sitter::Node<'tree>);
#[automatically_derived]
impl<'tree> RBrace<'tree> {}
#[automatically_derived]
impl<'tree> TryFrom<tree_sitter::Node<'tree>> for RBrace<'tree> {
    type Error = type_sitter_lib::IncorrectKind<'tree>;
    #[inline]
    fn try_from(node: tree_sitter::Node<'tree>) -> Result<Self, Self::Error> {
        if node.kind() == "}" {
            Ok(Self(node))
        } else {
            Err(type_sitter_lib::IncorrectKind {
                node,
                kind: <Self as type_sitter_lib::TypedNode<'tree>>::KIND,
            })
        }
    }
}
#[automatically_derived]
impl<'tree> type_sitter_lib::TypedNode<'tree> for RBrace<'tree> {
    const KIND: &'static str = "}";
    #[inline]
    fn node(&self) -> &tree_sitter::Node<'tree> {
        &self.0
    }
    #[inline]
    unsafe fn from_node_unchecked(node: tree_sitter::Node<'tree>) -> Self {
        Self(node)
    }
}
