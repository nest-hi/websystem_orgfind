
-- USER?

SELECT * from organizations;
SELECT * from users;
SELECT * from tags;
SELECT * from organization_tags;
SELECT * from events;

alter table organizations
ADD background_image text;

-- TODO Apr 24: a query for:
-- 1.getting the tags of an organization(by id or name)
-- 2.returning organizations based on tags



-- Joins for organization with it's tags
SELECT 
    o.id,
    o.name,
    t.id AS tag_id,
    t.name AS tag_name
FROM organizations o
LEFT JOIN organization_tags ot ON o.id = ot.organization_id
LEFT JOIN tags t ON ot.tag_id = t.id
WHERE t.name = 'Arts & Sciences';

update table tags 


-- 2,
CREATE TABLE events(
    id SERIAL PRIMARY KEY,
    title  TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    date_created timestamp not null default CURRENT_TIMESTAMP,
    date_occuring DATE,
    organization_id INTEGER NOT NULL,
    FOREIGN KEY (organization_id) REFERENCES organizations(id)
        ON DELETE CASCADE
);

INSERT into events(title , description, date_occuring, organization_id)
VALUES
('Gamejam', 'lorem basta ganon gamejam','10/25/2027' ,38),
('Hackathon', 'lorem basta hakerist','12/17/2026' ,38);

-- Tags table
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE
);

SELECT * from events;

SELECT id, title, description, date_occuring, organization_id FROM events;

INSERT INTO tags (name) VALUES
('Accounting'),
('Advocacy'),
('Agriculture'),
('Agricultural Engineering'),
('Anime'),
('Animals'),
('Arts'),
('Arts & Sciences'),
('Biology'),
('Broadcast'),
('Business'),
('Civil Engineering'),
('Cloud'),
('Communication'),
('Community'),
('Computer'),
('Creativity'),
('Criminology'),
('Debate'),
('Economics'),
('Education'),
('Electronics'),
('Emergency'),
('Engineering'),
('English'),
('Esports'),
('Finance'),
('Food Technology'),
('Gaming'),
('Governance'),
('Health'),
('HR'),
('Hospitality'),
('Industrial Engineering'),
('International'),
('International Relations'),
('IT'),
('Journalism'),
('Labor'),
('Language'),
('Law'),
('Leadership'),
('LGBTQ+'),
('Literature'),
('Management'),
('Marketing'),
('Mathematics'),
('Media'),
('Medical'),
('Medical Technology'),
('Mental Health'),
('Multimedia'),
('Music'),
('Nursing'),
('Operations'),
('Outdoor'),
('Performance'),
('Physical Activity'),
('Politics'),
('Pop Culture'),
('Programming'),
('Psychology'),
('Publication'),
('Religious'),
('Safety'),
('Scholars'),
('Science'),
('Security'),
('Service'),
('Social Science'),
('Social Work'),
('Sports'),
('STEM'),
('Sustainability'),
('Technology'),
('Tourism'),
('Volunteer'),
('Writing'),
('Youth'),
('Veterinary');


INSERT into tags (name) values ('Political Science');

select * from events;

-- Junction table for many-to-many relationship
CREATE TABLE organization_tags (
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
    PRIMARY KEY (organization_id, tag_id)
);

CREATE TABLE user_tags(
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE
);

CREATE TABLE event_tags(
    event_id INTEGER NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    tag_id INTEGER NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
);


-- INSERT INTO organizations (name) VALUES
--    ('Argumentatum El Debaztere Society'),
--    ("CCJ Information, Communications, and Technical Team"),
--    ("CCJ Emergency Response and Airsoft Team"),
--    ("CSPEAR Sports Organization"), 
--    ("CvSU DOST Scholars Association"),
--    ("The Hornets Cheer Squad"), 
--    ("CvSU Mountaineering Society"), 
--    ("CvSU Musikeros"),
--    ("CvSU Otaku Society"), 
--    ("CvSU Red Cross Youth"), 
--    ("CvSU Silayan"), 
--    ("AWS Learning Club - Spade"), 
--    ("Lipon ng Kagalingan ang Hanay ng mga Artistikong Indibidwal"),
--    ("The Asclepian Society"), 
--    ("The Gazette"), 
--    ("The Pinnacle Esports Organization"),
--    ("Youth for Animals"),
--    ("Foreign Student Association"), 
--    ("Gerero's Spear"),
--    ("CVSU Kasama  Kabsuhenyong Samahan ng Manggagawa Manlilikha at Manininda"),
--    ("Mind Your Mind"), 
--    ("Talesmiths' Collective"),
--    ("Sinagtala Multimedia Arts Organization"), 
--    ("Society for the Advancement of Veterinary Education and Research"), 
--    ("YTR Youth Organization Inc."), 
--    ("DZSU Hayag Luntian"), 
--    ("Kristyanong Kabataan para sa Bayan"),

-- --    

--    ("College of Arts and Sciences Student Council"), 
--    ("College of Criminal Justice Student Council"), 
--    ("CEMDS Student Council"), 
--    ("College of Education Student Council"), 
--    ("CEIT Student Council"), 
--    ("CAFENR Student Council"), 
--    ("College of Nursing Student Council"), 
--    ("CSPEAR Student Council"),
--    ("CTHM Student Council"), 
--    ("CVMBS Student Council"), 

-- -- 

--    ("Computer Science Student Organization"),
--    ("Computer Engineering Students' Society"),
--    ("Elite League of Information Technology"),
--    ("Criminology Society"), 
--    ("Industrial Security Student Society"),
--    ("Education Circle"), 
--    ("International Studies Students' Association"), 
--    ("Journalism Guild"), 
--    ("Junior Financial Executives"), 
--    ("Junior Marketing Association"), 
--    ("Junior Operations Management Association"), 
--    ("Junior People Management Association of the Philippines"), 
--    ("Junior Philippine Institution of Accountants"),
--    ("Junior Social Workers of the Philippines"), 
--    ("Leading Association of Nightingales in Training & Emerging Registered Nurses"), 
--    ("Matayuyon Crop Science Society"), 
--    ("Mitochondrion Society"), 
--    ("PAMANA"),  
--    ("PAFT"),  
--    ("Philippine Association of Students in Office Administration"), 
--    ("Philippine Institute of Civil Engineers"),  
--    ("Philippine Institute of Industrial Engineers"), 
--    ("Philippine Society of Medical Technology Students"), 
--    ("Philippine Society of Agricultural and Biosystems Engineers"), 
--    ("Pragmaticus"), 
--    ("Psychology Circle"), 
--    ("Radicand"), 
--    ("Rodeo Club"), 
--    ("Salinlahi"), 
--    ("Society of Future Economists"), 
--    ("Society of Industrial Technology Electronics Students"), 
--    ("Student Congress of Physical Education"), 
--    ("Tourism Students Association"), 
--    ("The United Architects of the Philippines Student Auxiliary"), 
--    ("UTOPIA"), 
--    ("VKV-VLV");



-- Organizations table
CREATE TABLE organizations (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL
);

-- Tags table
CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE
);


-- Junction table for tags
CREATE TABLE organization_tags (
    organization_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,

    PRIMARY KEY (organization_id, tag_id),

    FOREIGN KEY (organization_id)
        REFERENCES organizations(id)
        ON DELETE CASCADE,

    FOREIGN KEY (tag_id)
        REFERENCES tags(id)
        ON DELETE CASCADE
);

SELECT * from organization_tags;

INSERT INTO organizations (name) VALUES
   ('Argumentatum El Debaztere Society'),
   ('CCJ Information, Communications, and Technical Team'),
   ('CCJ Emergency Response and Airsoft Team'),
   ('CSPEAR Sports Organization'), 
   ('CvSU DOST Scholars Association'),
   ('The Hornets Cheer Squad'), 
   ('CvSU Mountaineering Society'), 
   ('CvSU Musikeros'),
   ('CvSU Otaku Society'), 
   ('CvSU Red Cross Youth'), 
   ('CvSU Silayan'), 
   ('AWS Learning Club - Spade'), 
   ('Lipon ng Kagalingan ang Hanay ng mga Artistikong Indibidwal'),
   ('The Asclepian Society'), 
   ('The Gazette'), 
   ('The Pinnacle Esports Organization'),
   ('Youth for Animals'),
   ('Foreign Student Association'), 
   ('Gerero''s Spear'),
   ('CVSU Kasama  Kabsuhenyong Samahan ng Manggagawa Manlilikha at Manininda'),
   ('Mind Your Mind'), 
   ('Talesmiths'' Collective'),
   ('Sinagtala Multimedia Arts Organization'), 
   ('Society for the Advancement of Veterinary Education and Research'), 
   ('YTR Youth Organization Inc.'), 
   ('DZSU Hayag Luntian'), 
   ('Kristyanong Kabataan para sa Bayan'),

   ('College of Arts and Sciences Student Council'), 
   ('College of Criminal Justice Student Council'), 
   ('CEMDS Student Council'), 
   ('College of Education Student Council'), 
   ('CEIT Student Council'), 
   ('CAFENR Student Council'), 
   ('College of Nursing Student Council'), 
   ('CSPEAR Student Council'),
   ('CTHM Student Council'), 
   ('CVMBS Student Council'), 

   ('Computer Science Student Organization'),
   ('Computer Engineering Students'' Society'),
   ('Elite League of Information Technology'),
   ('Criminology Society'), 
   ('Industrial Security Student Society'),
   ('Education Circle'), 
   ('International Studies Students'' Association'), 
   ('Journalism Guild'), 
   ('Junior Financial Executives'), 
   ('Junior Marketing Association'), 
   ('Junior Operations Management Association'), 
   ('Junior People Management Association of the Philippines'), 
   ('Junior Philippine Institution of Accountants'),
   ('Junior Social Workers of the Philippines'), 
   ('Leading Association of Nightingales in Training & Emerging Registered Nurses'), 
   ('Matayuyon Crop Science Society'), 
   ('Mitochondrion Society'), 
   ('PAMANA'),  
   ('PAFT'),  
   ('Philippine Association of Students in Office Administration'), 
   ('Philippine Institute of Civil Engineers'),  
   ('Philippine Institute of Industrial Engineers'), 
   ('Philippine Society of Medical Technology Students'), 
   ('Philippine Society of Agricultural and Biosystems Engineers'), 
   ('Pragmaticus'), 
   ('Psychology Circle'), 
   ('Radicand'), 
   ('Rodeo Club'), 
   ('Salinlahi'), 
   ('Society of Future Economists'), 
   ('Society of Industrial Technology Electronics Students'), 
   ('Student Congress of Physical Education'), 
   ('Tourism Students Association'), 
   ('The United Architects of the Philippines Student Auxiliary'), 
   ('UTOPIA'), 
   ('VKV-VLV');






WITH org_data(name, tags) AS (
    VALUES 
    ('Argumentatum El Debaztere Society', '["Debate", "Law", "Communication"]'::jsonb),
    ('CCJ Information, Communications, and Technical Team', '["Technology", "Communication", "IT"]'::jsonb),
    ('CCJ Emergency Response and Airsoft Team', '["Emergency", "Safety", "Service"]'::jsonb),
    ('CSPEAR Sports Organization', '["Sports", "Physical Activity"]'::jsonb), 
    ('CvSU DOST Scholars Association', '["Academic", "Scholars", "STEM"]'::jsonb),
    ('The Hornets Cheer Squad', '["Sports", "Performance"]'::jsonb), 
    ('CvSU Mountaineering Society', '["Outdoor", "Adventure"]'::jsonb), 
    ('CvSU Musikeros', '["Music", "Arts"]'::jsonb),
    ('CvSU Otaku Society', '["Anime", "Pop Culture"]'::jsonb), 
    ('CvSU Red Cross Youth', '["Health", "Volunteer", "Emergency"]'::jsonb), 
    ('CvSU Silayan', '["LGBTQ+", "Advocacy"]'::jsonb), 
    ('AWS Learning Club - Spade', '["Cloud", "Technology", "Programming"]'::jsonb), 
    ('Lipon ng Kagalingan ang Hanay ng mga Artistikong Indibidwal', '["Arts", "Creativity"]'::jsonb),
    ('The Asclepian Society', '["Medical", "Health"]'::jsonb), 
    ('The Gazette', '["Media", "Publication", "Journalism"]'::jsonb), 
    ('The Pinnacle Esports Organization', '["Gaming", "Esports", "Technology"]'::jsonb),
    ('Youth for Animals', '["Animals", "Advocacy"]'::jsonb),
    ('Foreign Student Association', '["International", "Community"]'::jsonb), 
    ('Gerero''s Spear', '["Sports", "Community"]'::jsonb),
    ('CVSU Kasama  Kabsuhenyong Samahan ng Manggagawa Manlilikha at Manininda', '["Labor", "Community", "Advocacy"]'::jsonb),
    ('Mind Your Mind', '["Mental Health", "Advocacy"]'::jsonb), 
    ('Talesmiths'' Collective', '["Writing", "Literature"]'::jsonb),
    ('Sinagtala Multimedia Arts Organization', '["Multimedia", "Arts", "Creative"]'::jsonb), 
    ('Society for the Advancement of Veterinary Education and Research', '["Veterinary", "Animals"]'::jsonb), 
    ('YTR Youth Organization Inc.', '["Religious", "Youth"]'::jsonb), 
    ('DZSU Hayag Luntian', '["Broadcast", "Media"]'::jsonb), 
    ('Kristyanong Kabataan para sa Bayan', '["Religious"]'::jsonb),

--    

    ('College of Arts and Sciences Student Council', '["Leadership", "Governance", "Arts & Sciences"]'::jsonb), 
    ('College of Criminal Justice Student Council', '["Leadership", "Criminology"]'::jsonb), 
    ('CEMDS Student Council', '["Leadership", "Business"]'::jsonb), 
    ('College of Education Student Council', '["Leadership", "Business"]'::jsonb), 
    ('CEIT Student Council', '["Leadership", "Technology", "Engineering", "IT"]'::jsonb), 
    ('CAFENR Student Council', '["Leadership", "Environment", "Agriculture"]'::jsonb), 
    ('College of Nursing Student Council', '["Leadership", "Health"]'::jsonb), 
    ('CSPEAR Student Council', '["Leadership", "Sports"]'::jsonb),
    ('CTHM Student Council', '["Leadership", "Tourism", "Hospitality"]'::jsonb), 
    ('CVMBS Student Council', '["Leadership", "Veterinary", "Medical"]'::jsonb), 

-- academic

    ('Computer Science Student Organization', '["Programming", "Technology", "Computer"]'::jsonb),
    ('Computer Engineering Students'' Society', '["Engineering", "Technology", "Computer"]'::jsonb),
    ('Elite League of Information Technology', '["Technology", "IT"]'::jsonb),
    ('Criminology Society', '["Law", "Criminology"]'::jsonb), 
    ('Industrial Security Student Society', '["Security", "Criminology"]'::jsonb),
    ('Education Circle', '["Education"]'::jsonb), 
    ('International Studies Students'' Association', '["International Relations", "Politics"]'::jsonb), 
    ('Journalism Guild', '["Media", "Writing"]'::jsonb), 
    ('Junior Financial Executives', '["Finance", "Business"]'::jsonb), 
    ('Junior Marketing Association', '["Marketing", "Business"]'::jsonb), 
    ('Junior Operations Management Association', '["Business", "Operations"]'::jsonb), 
    ('Junior People Management Association of the Philippines', '["HR", "Management"]'::jsonb), 
    ('Junior Philippine Institution of Accountants', '["Accounting", "Finance"]'::jsonb),
    ('Junior Social Workers of the Philippines', '["Social Work"]'::jsonb), 
    ('Leading Association of Nightingales in Training & Emerging Registered Nurses', '["Nursing", "Health"]'::jsonb), 
    ('Matayuyon Crop Science Society', '["Agriculture"]'::jsonb), 
    ('Mitochondrion Society', '["Biology", "Science"]'::jsonb), 
    ('PAMANA', '["Social Science"]'::jsonb),  
    ('PAFT', '["Food Technology"]'::jsonb),  
    ('Philippine Association of Students in Office Administration', '["Leadership","Service","Advocacy"]'::jsonb), 
    ('Philippine Institute of Civil Engineers', '["Civil Engineering"]'::jsonb),  
    ('Philippine Institute of Industrial Engineers', '["Industrial Engineering"]'::jsonb), 
    ('Philippine Society of Medical Technology Students', '["Medical Technology"]'::jsonb), 
    ('Philippine Society of Agricultural and Biosystems Engineers', '["Agricultural Engineering"]'::jsonb), 
    ('Pragmaticus', '["English", "Language"]'::jsonb), 
    ('Psychology Circle', '["Psychology", "Mental Health"]'::jsonb), 
    ('Radicand', '["Mathematics"]'::jsonb), 
    ('Rodeo Club', '["Agriculture", "Animals"]'::jsonb), 
    ('Salinlahi', '["Environment", "Sustainability"]'::jsonb), 
    ('Society of Future Economists', '["Economics"]'::jsonb), 
    ('Society of Industrial Technology Electronics Students', '["Electronics", "Technology"]'::jsonb), 
    ('Student Congress of Physical Education', '["Sports", "Education"]'::jsonb), 
    ('Tourism Students Association', '["Tourism"]'::jsonb), 
    ('The United Architects of the Philippines Student Auxiliary', '["Architecture"]'::jsonb), 
    ('UTOPIA', '["Politics"]'::jsonb), 
    ('VKV-VLV', '["Veterinary"]'::jsonb)
),
expanded AS (
    SELECT 
        name,
        jsonb_array_elements_text(tags) AS tag_name
    FROM org_data
)

INSERT INTO organization_tags (organization_id, tag_id)
SELECT 
    o.id,
    t.id
FROM expanded e
JOIN organizations o ON o.name = e.name
JOIN tags t ON t.name = e.tag_name ON CONFLICT (organization_id, tag_id) DO NOTHING;

SELECT * from tags;

SELECT t.*
FROM tags t
LEFT JOIN organization_tags ot ON ot.tag_id = t.id
WHERE ot.tag_id IS NULL;

INSERT INTO tags (name) VALUES ('Architecture');