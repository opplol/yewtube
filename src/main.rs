use gloo_net::http::Request;
use video::Video;
use yew::prelude::*;

mod video;

#[derive(Properties, PartialEq)]
struct VideosListProps {
    videos: Vec<Video>,
    on_click: Callback<Video>,
}

#[function_component(VideosList)]
fn video_list(VideosListProps { videos, on_click }: &VideosListProps) -> Html {
    let on_click = on_click.clone();
    videos
        .iter()
        .map(|video| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = video.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };
            html! {
                <p key={video.id} onclick={on_video_select}>{format!("{}: {}", video.speaker, video.title)}</p>
            }
        })
        .collect()
}

#[derive(Properties, PartialEq)]
struct VideoDetailsProps {
    video: Video,
}

#[function_component(VideoDetails)]
fn video_details(VideoDetailsProps { video }: &VideoDetailsProps) -> Html {
    html! {
        <div>
            <h3>{ video.title.clone() }</h3>
            <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    // let videos = vec![
    //     Video {
    //         id: 1,
    //         title: "building and breaking things".to_string(),
    //         speaker: "John Doe".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 2,
    //         title: "The development process".to_string(),
    //         speaker: "Jane Smith".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 3,
    //         title: "The Web 7.0".to_string(),
    //         speaker: "Matt Miller".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    //     Video {
    //         id: 4,
    //         title: "Mouseless development".to_string(),
    //         speaker: "Tome Jerry".to_string(),
    //         url: "https://youtu.be/PsaFVLr8t4E".to_string(),
    //     },
    // ];

    let videos = use_state(|| vec![]);
    {
        let videos = videos.clone();
        use_effect_with_deps(
            move |_| {
                let videos = videos.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_videos: Vec<Video> =
                        Request::get("https://yew.rs/tutorial/data.json")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();
                    videos.set(fetched_videos)
                });
                || ()
            },
            (),
        );
    }

    // let videos = videos
    //     .iter()
    //     .map(|video| {
    //         html! {
    //             <p key={video.id}>{format!("{}: {}", video.speaker, video.title)}</p>
    //         }
    //     })
    //     .collect::<Html>();

    let selected_video = use_state(|| None);

    let on_video_select = {
        let selected_video = selected_video.clone();
        Callback::from(move |video: Video| selected_video.set(Some(video)))
    };

    let details = selected_video.as_ref().map(|video| {
        html! {
            <VideoDetails video={video.clone()} />
        }
    });
    html! {
        <>
            <h1>{ "RustConf Explorer" }</h1>
            <div>
                <h3>{ "Videos to watch" }</h3>
                // { videos }
                <VideosList videos={(*videos).clone()} on_click={on_video_select.clone()} />
            </div>
            { for details }

            // <div>
            //     <h3>{ "John Doe: Building and breaking things" }</h3>
            //     <img src="https://via.placeholder.com/640x360.png?text=Video+Player+Placeholder" alt="video thumbnail" />
            // </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
