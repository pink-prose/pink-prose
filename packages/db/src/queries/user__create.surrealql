begin transaction;

if (select * from users where email = $email) {
	throw "Email already taken";
};

if !string::is::email($email) {
	throw "Invalid email";
};

create users:ulid() set
	email = $email,
	username = $username
;

commit transaction;
