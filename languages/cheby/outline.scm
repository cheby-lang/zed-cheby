(function_declaration
  (visibility_modifier)? @context
  "fn" @context
  name: (_) @name) @item

(type_declaration
  (visibility_modifier)? @context
  "exposed"? @context
  "type" @context
  name: (_) @name) @item

(variant
  name: (_) @name) @item

(field_declaration
  name: (_) @name) @item

(type_alias
  (visibility_modifier)? @context
  "type" @context
  name: (_) @name) @item

(const_declaration
  (visibility_modifier)? @context
  "const" @context
  name: (_) @name) @item

(interface_declaration
  (visibility_modifier)? @context
  "interface" @context
  name: (_) @name) @item

(interface_function
  "fn" @context
  name: (_) @name) @item

(test_declaration
  "test" @context
  name: (_) @name) @item
