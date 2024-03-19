-- TODO: replace schemaless with schemafull when closer to stability
-- TODO: find some way to enforce ULID?
begin transaction;

let $target_schema_version = 1;
let $current_schema_version = (
	select current_schema_version from db_meta:schema_info
);

-- newly created db
if !$current_schema_version {
	create db_meta:schema_info set
		current_schema_version = $target_schema_version
	;

	define table users schemaless;

	define field email    on users type string
		assert string::is::email($value);
	define field username on users type string; -- TODO: char limit?
	define field bio      on users type option<string>; -- TODO: char limit?
	define field display_name on users type option<string>; -- TODO: char limit?

	define index email_unique    on table users columns email unique;
	define index username_unique on table users columns email unique;

	define table stories schemaless;

	define field title on stories type string; -- TODO: char limit?
	define field date_modified on stories type datetime;
	-- none if not published, some if published
	define field date_published on stories type option<datetime>;
	define field cover_url on stories type option<string> -- TODO: char limit?????????
		assert string::is::url($value);
	-- TODO: string is 6 bytes but i64 is 8, and also then there wouldn't need to be conversion hmmmmmmm?
	define field cover_loading_colour on stories type int;
	define field completion_status on stories type string -- enum in rust
		assert function() {
			// surely I don't need to do this weird javascript thing
			return "done incomplete hiatus cancelled".split(" ").contains(this.value)
		};
	define field content_rating on stories type string; -- enum in rust, TODO: validate
};

-- increment target version on top of file every time a modification
-- is made, and then do the updates. this file will run on db
-- startup / connect to make sure everything is up to date

commit transaction;
