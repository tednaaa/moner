CREATE TABLE IF NOT EXISTS users (
	id BIGSERIAL PRIMARY KEY,
	email VARCHAR(255) NOT NULL UNIQUE,
	username VARCHAR(100) NOT NULL UNIQUE,
	password VARCHAR(255) NOT NULL,
	is_verified BOOLEAN NOT NULL DEFAULT FALSE,
	created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE skills (
	id BIGSERIAL PRIMARY KEY,
	name VARCHAR(255) NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS user_skills (
	user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	skill_id BIGINT NOT NULL REFERENCES skills (id) ON DELETE CASCADE,
	PRIMARY KEY (user_id, skill_id)
);

CREATE TABLE IF NOT EXISTS experience (
	id BIGSERIAL PRIMARY KEY,
	user_id BIGINT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	company_name VARCHAR(255) NOT NULL,
	occupation VARCHAR(255) NOT NULL,
	location_name VARCHAR(255) NOT NULL,
	location_type VARCHAR(50) NOT NULL CHECK (location_type IN ('on-site', 'remote', 'hybrid')),
	employment_type VARCHAR(50) NOT NULL CHECK (
		employment_type IN (
			'full-time',
			'part-time',
			'contract',
			'freelance',
			'internship'
		)
	),
	start_date TIMESTAMPTZ NOT NULL,
	end_date TIMESTAMPTZ,
	is_current BOOLEAN NOT NULL DEFAULT FALSE,
	description TEXT NOT NULL
);

INSERT INTO
	users (email, username, password, is_verified)
VALUES
	(
		'tednaaa@gmail.com',
		'tednaaa',
		'$argon2id$v=19$m=19456,t=2,p=1$BtOCIz2abhoTV0HU8U7YOA$nXpfAv7ubnOFP1TF+Ym00zj83l+J6+/Kc1W1ZNFooDQ',
		true
	);

INSERT INTO
	skills (name)
VALUES
	('Rust'),
	('Go'),
	('Python'),
	('Javascript'),
	('Typescript'),
	('UI/UX'),
	('Graphic Design'),
	('Product Management'),
	('Digital Marketing'),
	('Data Analytics');

INSERT INTO
	user_skills (user_id, skill_id)
VALUES
	(1, 1),
	(1, 2),
	(1, 3),
	(1, 4),
	(1, 5);

INSERT INTO
	experience (
		user_id,
		company_name,
		occupation,
		location_name,
		location_type,
		employment_type,
		start_date,
		end_date,
		description
	)
VALUES
	(
		1,
		'Google',
		'Software Engineer',
		'Mountain View, CA',
		'remote',
		'full-time',
		'2020-01-01 00:00:00',
		'2021-01-01 00:00:00',
		'I love coding!'
	);

INSERT INTO
	experience (
		user_id,
		company_name,
		occupation,
		location_name,
		location_type,
		employment_type,
		start_date,
		end_date,
		description
	)
VALUES
	(
		1,
		'Facebook',
		'Software Engineer',
		'Menlo Park, CA',
		'remote',
		'full-time',
		'2020-05-01 00:00:00',
		'2021-01-01 00:00:00',
		'I love coding!'
	);

INSERT INTO
	experience (
		user_id,
		company_name,
		occupation,
		location_name,
		location_type,
		employment_type,
		start_date,
		end_date,
		description
	)
VALUES
	(
		1,
		'Apple',
		'Software Engineer',
		'Cupertino, CA',
		'on-site',
		'full-time',
		'2018-01-01 00:00:00',
		'2021-05-01 00:00:00',
		'I love coding!'
	);
