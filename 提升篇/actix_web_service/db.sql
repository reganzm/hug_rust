CREATE TABLE IF NOT EXISTS course
(
    id integer NOT NULL DEFAULT nextval('course_id_seq'::regclass),
    teacher_id integer NOT NULL,
    name character varying(140) COLLATE pg_catalog."default" NOT NULL,
    "time" timestamp without time zone DEFAULT now(),
    description character varying(2000) COLLATE pg_catalog."default",
    format character varying(30) COLLATE pg_catalog."default",
    structure character varying(200) COLLATE pg_catalog."default",
    duration character varying(30) COLLATE pg_catalog."default",
    price integer,
    language character varying(30) COLLATE pg_catalog."default",
    level character varying(30) COLLATE pg_catalog."default",
    CONSTRAINT course_pkey PRIMARY KEY (id)
)
