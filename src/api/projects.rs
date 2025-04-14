use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub image_url: String,
    pub case_study_url: String,
}

pub fn get_projects() -> Vec<Project> {
    vec![
        Project {
            id: "project-1".to_string(),
            title: "Portfolio Website".to_string(),
            category: "Web Application".to_string(),
            description: "A portfolio website for an academic, made with Deno Fresh, Postgresql and Supabase.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Ftse4.mm.bing.net%2Fth%3Fid%3DOIP._46ZR0zkVmoRwaMuSdDFXQHaEo%26pid%3DApi&f=1&ipt=aff04108aa1e9a287c52a3584357be0c54b95167122f03357e1de7886be0c0bd".to_string(),
            case_study_url: "https://woodrock.deno.dev/".to_string(),
        },
        Project {
            id: "project-2".to_string(),
            title: "Wellington Bus Timetable".to_string(),
            category: "Web Application".to_string(),
            description: "A bus timetable for the Wellington Region, made with Deno Fresh and Metlink API.".to_string(),
            image_url: "https://media.gettyimages.com/id/1501353205/photo/double-decker-metlink-bus-te-whanganui-a-tara-wellington-spring-day.jpg?s=2048x2048&w=gi&k=20&c=aIXWGu9Hu8TAfdf4bLmj22whHJVtGV12WujZ80WkOtk=".to_string(),
            case_study_url: "https://bus-timetable.deno.dev/".to_string(),
        },
        Project {
            id: "project-3".to_string(),
            title: "Renewed Roots".to_string(),
            category: "Web Application".to_string(),
            description: "An online shop for selling recycled furniture, made with Deno Fresh".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwww.architectureartdesigns.com%2Fwp-content%2Fuploads%2F2016%2F03%2F18-Remarkable-Furniture-Designs-Made-From-Recycled-Pallet-Wood-13.jpg&f=1&nofb=1&ipt=452ac8479c837abdd46a689d9a2f630bfca9caf44cfd8844be6acec775925037".to_string(),
            case_study_url: "https://flogging-furniture.deno.dev/".to_string(),
        },
        Project {
            id: "project-4".to_string(),
            title: "Ionic Scholar".to_string(),
            category: "Mobile Application".to_string(),
            description: "A citations tracker for academics made with Ionic React Framework.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Funissa.edu.bn%2Fjournal%2Fpublic%2Fsite%2Fimages%2Froot%2Fgoogle-scholar-d727840d5d57573b2c18a8dcb280ee96.png&f=1&nofb=1&ipt=2a69517dd0930eb0e4445b2568b7b3b2b026fa4c4c5a2f3efc5f35c6c2807370".to_string(),
            case_study_url: "https://github.com/woodRock/ionic-scholar".to_string(),
        },
        Project {
            id: "project-5".to_string(),
            title: "Wordle Solver Transformer".to_string(),
            category: "Artificial Intelligence".to_string(),
            description: "A worlde solver built with python, pytorch, and deno fresh".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fgamingonphone.com%2Fwp-content%2Fuploads%2F2022%2F01%2FScreenshot-2022-01-11-at-7.49.02-PM-scaled.jpg&f=1&nofb=1&ipt=17fcf0024fd2a58185820e3fe994be8e4509de7755084e783bf8883ab3c14be1".to_string(),
            case_study_url: "https://github.com/woodRock/wordle".to_string(),
        },
        Project {
            id: "project-6".to_string(),
            title: "Autograd".to_string(),
            category: "Artificial Intelligence".to_string(),
            description: "Deep learning library written in c++ and cuda for a basic autograd.".to_string(),
            image_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fsslprod.oss-cn-shanghai.aliyuncs.com%2Fstable%2Fslides%2F04_Back_propagation_and_PyTorch_autograd%2F04_Back_propagation_and_PyTorch_autograd_1440-05.jpg&f=1&nofb=1&ipt=10660cae9b14ef211218747f70091d219345ee03a81a1007bc6e66183d948de3".to_string(),
            case_study_url: "https://github.com/woodRock/autograd".to_string(),
        }
    ]
}