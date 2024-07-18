begin transaction;

define table if not exists dbinfo schemaless;

if !(select * from dbinfo:dbinfo) {
	create dbinfo:dbinfo set version = 0
};

let $dbinfo = (select * from dbinfo:dbinfo)[0];

commit transaction;
