use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct SomeThingPublic {
//     name: String,
//     // duplicated field needs to be deserialized in a different way
//     groups: HashMap<String, HashSet<Year>>,
// }

#[derive(Debug, Deserialize)]
pub struct DefinedAgeGroups {
    pub age_groups: Option<Vec<AgeGroup>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AgeGroup {
    pub age_group_name: String,
    #[serde(rename = "$value")]
    pub year: Vec<String>,
}

// fn year<'de, D>(deserializer: D) -> Result<Year, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     let s: String = Deserialize::deserialize(deserializer)?;
//     match s.parse::<i64>() {
//         Ok(year) => Ok(datetime::Year(year)),
//         Err(err) => Err(serde::de::Error::custom(err.to_string())),
//     }
// }

// cannot create a deserializer for Year type because its not mine.

// fn deserializer<'de, D>(deserializer: D) -> Result<HashMap<String, HashSet<Year>>, D::Error>
// where
//     D: serde::de::Deserializer<'de>,
// {
//     const EXPECTED: &str = "4 character long string that is parsable to a year.";
//
//     let age_group_name: Vec<DefinedAgeGroups> = serde::de::Deserialize::deserialize(deserializer)?;
//
//     dbg!(age_group_name);
//
//     todo!();
// }

/*
Example underlying data
<DefinedAgeGroups>
        <AgeGroup>
            <AgeGroupName>Klasse 91-03</AgeGroupName>
            <Year> 1991</Year>
            <Year> 1992</Year>
            <Year> 1993</Year>
            <Year> 1994</Year>
            <Year> 1995</Year>
            <Year> 1996</Year>
            <Year> 1997</Year>
            <Year> 1998</Year>
            <Year> 1999</Year>
            <Year> 2000</Year>
            <Year> 2001</Year>
            <Year> 2002</Year>
            <Year> 2003</Year>
        </AgeGroup>
        <AgeGroup>
            <AgeGroupName>Klasse 04-05</AgeGroupName>
            <Year> 2004</Year>
            <Year> 2005</Year>
        </AgeGroup>
        <AgeGroup>
            <AgeGroupName>Klasse 06-07</AgeGroupName>
            <Year> 2006</Year>
            <Year> 2007</Year>
        </AgeGroup>
        <AgeGroup>
            <AgeGroupName>Klasse 08</AgeGroupName>
            <Year> 2008</Year>
        </AgeGroup>
        <AgeGroup>
            <AgeGroupName>Klasse 09</AgeGroupName>
            <Year> 2009</Year>
        </AgeGroup>
        <AgeGroup>
            <AgeGroupName>F</AgeGroupName>
        </AgeGroup>
    </DefinedAgeGroups>

*/
