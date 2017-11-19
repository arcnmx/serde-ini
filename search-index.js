var searchIndex = {};
searchIndex["serde_ini"] = {"doc":"Windows INI format serialization for serde","items":[[0,"de","serde_ini","",null,null],[3,"Deserializer","serde_ini::de","",null,null],[3,"SectionDeserializer","","",null,null],[3,"ValueDeserializer","","",null,null],[4,"Error","","",null,null],[13,"Custom","","",0,null],[13,"UnexpectedEof","","",0,null],[13,"InvalidState","","",0,null],[6,"Result","","",null,null],[8,"Trait","","",null,null],[10,"next","","",1,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"error"}}],[11,"from","","",0,{"inputs":[{"name":"parseinterror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"parsefloaterror"}],"output":{"name":"self"}}],[11,"from","","",0,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"custom","","",0,{"inputs":[{"name":"t"}],"output":{"name":"self"}}],[11,"fmt","","",2,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",2,{"inputs":[{"name":"t"}],"output":{"name":"self"}}],[0,"ser","serde_ini","",null,null],[3,"Serializer","serde_ini::ser","",null,null],[3,"MapSerializer","","",null,null],[4,"UnsupportedType","","",null,null],[13,"Bool","","",3,null],[13,"Bytes","","",3,null],[13,"None","","",3,null],[13,"Unit","","",3,null],[13,"Seq","","",3,null],[13,"Map","","",3,null],[4,"Error","","",null,null],[13,"Custom","","",4,null],[13,"UnsupportedType","","",4,null],[13,"NonStringKey","","",4,null],[13,"TopLevelMap","","",4,null],[13,"OrphanValue","","",4,null],[13,"MapKeyMissing","","",4,null],[6,"Result","","",null,null],[11,"clone","","",3,{"inputs":[{"name":"self"}],"output":{"name":"unsupportedtype"}}],[11,"fmt","","",3,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",4,{"inputs":[{"name":"self"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"error"}],"output":{"name":"self"}}],[11,"from","","",4,{"inputs":[{"name":"unsupportedtype"}],"output":{"name":"self"}}],[11,"fmt","","",4,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",4,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"custom","","",4,{"inputs":[{"name":"t"}],"output":{"name":"self"}}],[11,"new","","",5,{"inputs":[{"name":"writer"}],"output":{"name":"self"}}],[11,"serialize_key","","",6,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"result"}}],[11,"serialize_value","","",6,{"inputs":[{"name":"self"},{"name":"t"}],"output":{"name":"result"}}],[11,"end","","",6,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"serialize_field","","",6,{"inputs":[{"name":"self"},{"name":"str"},{"name":"t"}],"output":{"name":"result"}}],[11,"end","","",6,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[0,"parse","serde_ini","",null,null],[3,"Parser","serde_ini::parse","",null,null],[3,"OkIter","","",null,null],[12,"0","","",7,null],[4,"Item","","",null,null],[13,"Empty","","",8,null],[13,"Section","","",8,null],[12,"name","serde_ini::parse::Item","",8,null],[13,"Value","serde_ini::parse","",8,null],[12,"key","serde_ini::parse::Item","",8,null],[12,"value","","",8,null],[13,"Comment","serde_ini::parse","",8,null],[12,"text","serde_ini::parse::Item","",8,null],[4,"SyntaxError","serde_ini::parse","",null,null],[13,"SectionNotClosed","","",9,null],[13,"SectionName","","",9,null],[13,"MissingEquals","","",9,null],[4,"Error","","",null,null],[13,"Inner","","",10,null],[13,"Syntax","","",10,null],[11,"clone","","",8,{"inputs":[{"name":"self"}],"output":{"name":"item"}}],[11,"eq","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"ne","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"cmp","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"option"}}],[11,"lt","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"le","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"gt","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"ge","","",8,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"bool"}}],[11,"fmt","","",8,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",9,{"inputs":[{"name":"self"}],"output":{"name":"syntaxerror"}}],[11,"eq","","",9,{"inputs":[{"name":"self"},{"name":"syntaxerror"}],"output":{"name":"bool"}}],[11,"cmp","","",9,{"inputs":[{"name":"self"},{"name":"syntaxerror"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",9,{"inputs":[{"name":"self"},{"name":"syntaxerror"}],"output":{"name":"option"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",9,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",10,{"inputs":[{"name":"self"}],"output":{"name":"error"}}],[11,"eq","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"ne","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"cmp","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"ordering"}}],[11,"partial_cmp","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"option"}}],[11,"lt","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"le","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"gt","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"ge","","",10,{"inputs":[{"name":"self"},{"name":"error"}],"output":{"name":"bool"}}],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",10,{"inputs":[{"name":"e"}],"output":{"name":"self"}}],[11,"fmt","","",10,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",10,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"cause","","",10,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"new","","",11,{"inputs":[{"name":"t"}],"output":{"name":"self"}}],[11,"into_inner","","",11,{"inputs":[{"name":"self"}],"output":{"name":"t"}}],[11,"from_str","","",11,{"inputs":[{"name":"str"}],"output":{"name":"self"}}],[11,"from_bufread","","",11,{"inputs":[{"name":"r"}],"output":{"name":"self"}}],[11,"from_read","","",11,{"inputs":[{"name":"r"}],"output":{"name":"self"}}],[11,"next","","",11,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[11,"next","","",7,{"inputs":[{"name":"self"}],"output":{"name":"option"}}],[0,"write","serde_ini","",null,null],[3,"Writer","serde_ini::write","",null,null],[4,"LineEnding","","",null,null],[13,"Linefeed","","",12,null],[13,"CrLf","","",12,null],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",12,{"inputs":[{"name":"self"}],"output":{"name":"lineending"}}],[11,"eq","","",12,{"inputs":[{"name":"self"},{"name":"lineending"}],"output":{"name":"bool"}}],[11,"partial_cmp","","",12,{"inputs":[{"name":"self"},{"name":"lineending"}],"output":{"name":"option"}}],[11,"cmp","","",12,{"inputs":[{"name":"self"},{"name":"lineending"}],"output":{"name":"ordering"}}],[11,"hash","","",12,null],[11,"default","","",12,{"inputs":[],"output":{"name":"self"}}],[11,"fmt","","",12,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",13,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"clone","","",13,{"inputs":[{"name":"self"}],"output":{"name":"writer"}}],[11,"new","","",13,{"inputs":[{"name":"w"},{"name":"lineending"}],"output":{"name":"self"}}],[11,"into_inner","","",13,{"inputs":[{"name":"self"}],"output":{"name":"w"}}],[11,"write","","",13,{"inputs":[{"name":"self"},{"name":"item"}],"output":{"name":"result"}}]],"paths":[[4,"Error"],[8,"Trait"],[3,"Deserializer"],[4,"UnsupportedType"],[4,"Error"],[3,"Serializer"],[3,"MapSerializer"],[3,"OkIter"],[4,"Item"],[4,"SyntaxError"],[4,"Error"],[3,"Parser"],[4,"LineEnding"],[3,"Writer"]]};
initSearch(searchIndex);