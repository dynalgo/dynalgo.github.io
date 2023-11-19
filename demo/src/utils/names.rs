/// Names (char type) generator

pub struct Names {}

impl Names {
    /// Returns a list of emoticons (304 characters maximum)
    pub fn emoticon(length: usize) -> Result<Vec<char>, String> {
        let emo1: Vec<char> = ('😀'..='🙏').collect();
        let emo2: Vec<char> = ('🤠'..='🧿').collect();
        let mut chars = Vec::with_capacity(emo1.len() + emo2.len());
        chars.extend(&emo1);
        chars.extend(&emo2);

        if length > chars.len() {
            Err(format!(
                "The number of emoticon names requested is too high (max {})",
                chars.len()
            ))?;
        }

        Ok(chars.into_iter().take(length).collect())
    }
    /*
    /// Returns the letters of the Latin alphabet (52 characters maximum)
    pub fn latin(length: usize) -> Result<Vec<char>, String> {
        let latin_lower: Vec<char> = ('a'..='z').collect();
        let latin_upper: Vec<char> = ('A'..='Z').collect();
        let mut chars = Vec::with_capacity(latin_lower.len() + latin_upper.len());
        chars.extend(&latin_upper);
        chars.extend(&latin_lower);

        if length > chars.len() {
            Err(format!(
                "The number of latin names requested is too high (max {})",
                chars.len()
            ))?;
        }

        Ok(chars.into_iter().take(length).collect())
    }

    /// Returns the letters of the Greek alphabet (50 characters maximum)
    pub fn greek(length: usize) -> Result<Vec<char>, String> {
        let greek_lower: Vec<char> = ('α'..='ω').collect();
        let greek_upper: Vec<char> = ('Α'..='Ω').collect();
        let mut chars = Vec::with_capacity(greek_lower.len() + greek_upper.len());
        chars.extend(&greek_lower);
        chars.extend(&greek_upper);

        if length > chars.len() {
            Err(format!(
                "The number of greek names requested is too high (max {})",
                chars.len()
            ))?;
        }

        Ok(chars.into_iter().take(length).collect())
    }
    */
}
