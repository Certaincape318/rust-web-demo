CREATE SEQUENCE user_seq1
  INCREMENT 2
  MINVALUE 1
  MAXVALUE 10000000
  START 3
  CACHE 1;
 


CREATE TABLE t_user
(
  id integer NOT NULL DEFAULT nextval('user_seq1'::regclass),
  name character varying(30),
  email character varying(100),
  remember_token character varying(100)
);
 
INSERT INTO t_user(  name, email, remember_token)  VALUES ('name1','email1','token1');
INSERT INTO t_user(  name, email, remember_token)  VALUES ('name2','email2','token2');
