CREATE TABLE course (
    course_id INTEGER PRIMARY KEY,
    course_name TEXT NOT NULL
);

CREATE TABLE enrolment (
    student_id INTEGER NOT NULL,
    course_id INTEGER NOT NULL,
    grade INTEGER,

    PRIMARY KEY (student_id, course_id),

    FOREIGN KEY (student_id)
        REFERENCES student(student_id),

    FOREIGN KEY (course_id)
        REFERENCES course(course_id)
);