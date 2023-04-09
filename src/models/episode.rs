pub struct Episode {
	pub uuid: String,
	pub enclosure: Enclosure,
    pub link: String,
    pub image: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub duration: i8,
    pub pub_date: String
}

pub struct Enclosure {
    pub url: String,
    pub length: i8,
    pub enclosure_type: String,
}
