create table bookmarks (
	identifier INTEGER PRIMARY KEY, 
	url TEXT, 
	description TEXT);

create table tags (
	identifier INTEGER PRIMARY KEY, 
	tag TEXT);
    
create table bookmarks_tags (
	identifier INTEGER PRIMARY KEY,
	bookmark INTEGER REFERENCES bookmarks(identifier), 
	tag INTEGER REFERENCES tags(identifier));

