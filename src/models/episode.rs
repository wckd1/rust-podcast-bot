pub struct Episode {
	pub uuid: String,
	pub enclosure: Enclosure,
    pub link: String,
    pub image: String,
    pub title: String,
    pub description: String,
    pub author: String,
    pub duration: u64,
    pub pub_date: String
}

pub struct Enclosure {
    pub url: String,
    pub length: u64,
    pub enclosure_type: String,
}
