use simple_xml_serialize::XMLElement;

use super::episode::Episode;

pub struct RSSFeed {
    items: Vec<Episode>
}

impl RSSFeed {
    pub fn init(items: Vec<Episode>) -> Self {
        Self { items }
    }

    pub fn build(&self) -> XMLElement {
        let mut rss = XMLElement::new("rss");
        rss.add_attr("version", "2.0");
        rss.add_attr("xmlns:itunes", "http://www.itunes.com/dtds/podcast-1.0.dtd");

        let mut channel = XMLElement::new("channel");

        channel.add_element(XMLElement::new("title").text("Private RSS feed"));
        channel.add_element(XMLElement::new("description").text("Generated by tg-youtube-podcasts-bot"));
        channel.add_element(XMLElement::new("itunes:image").text("https://www.clipartkey.com/mpngs/m/197-1971515_youtube-music-seamless-audio-video-switching-transparent-youtube.png"));
        channel.add_element(XMLElement::new("language").text("ru"));
        channel.add_element(XMLElement::new("itunes:explicit").text("false"));
        channel.add_element(XMLElement::new("itunes:category").text("Education"));

        let items: Vec<XMLElement> = self.items.iter().map(|ep| {
            let mut enclosure = XMLElement::new("enclosure");
            enclosure.add_attr("url", &ep.enclosure.url);
            enclosure.add_attr("length", &ep.enclosure.length);
            enclosure.add_attr("type", &ep.enclosure.enclosure_type);

            let desc = "<![CDATA[".to_string() + &ep.description + "]]>";

            let mut el = XMLElement::new("item");
            el.add_element(XMLElement::new("guid").text(&ep.uuid));
            el.add_element(enclosure);
            el.add_element(XMLElement::new("link").text(&ep.link));
            el.add_element(XMLElement::new("image").text(&ep.image));
            el.add_element(XMLElement::new("title").text(&ep.title));
            el.add_element(XMLElement::new("description").text(desc));
            el.add_element(XMLElement::new("author").text(&ep.author));
            el.add_element(XMLElement::new("duration").text(&ep.duration));
            el.add_element(XMLElement::new("pub_date").text(&ep.pub_date));

            return el
        }).collect();

        channel.add_elements(items);
        rss.add_element(channel);

        return rss;
    }
}