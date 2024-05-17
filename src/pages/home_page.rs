use leptos::{html::Section, *};
use leptos_use::{
    use_intersection_observer, use_intersection_observer_with_options,
    UseIntersectionObserverOptions,
};
use stylers::*;

use crate::{
    components::{
        card_switcher, CardSwitcher, CardSwitcherSection, JobCard, TechCard, TitleHeader,
    },
    model::{
        job::{jobs_mock_data, Job},
        tech_item::{mock_tech_data, TechType},
    },
};

#[component]
pub fn HomePage() -> impl IntoView {
    let styles = style_sheet!("./src/pages/home_page.scss");

    let start_date = chrono::NaiveDate::from_ymd_opt(2020, 02, 01).unwrap();
    let today = chrono::Local::now().date_naive();

    let years = (today - start_date).num_weeks() as f32 / 52.0;

    let years_str = if years.round() > years {
        format!("almost {} years", years.round())
    } else {
        format!("more than {} years", years.round())
    };

    let jobs = jobs_mock_data()
        .iter()
        .map(|job| view! { <JobCard job={job.clone()}/> })
        .collect::<Vec<_>>();

    let crate_version = env!("CARGO_PKG_VERSION");

    let create_section_animation = || {
        let section_ref = create_node_ref::<Section>();
        use_intersection_observer(section_ref, |entries, observer| {
            if entries[0].is_intersecting() {
                let _ = entries[0].target().class_list().add_1("animated");
                let _ = entries[0].target().class_list().remove_1("hidden");
                observer.unobserve(&entries[0].target());
            }
        });
        section_ref
    };

    let _who_am_i_ref = create_section_animation();
    let _what_do_i_do_ref = create_section_animation();
    let _my_last_jobs_ref = create_section_animation();
    let _cool_stuff_ref = create_section_animation();
    let _be_rust_ref = create_section_animation();
    let _be_cpp_ref = create_section_animation();
    let _ray_man_ref = create_section_animation();
    let _skillset_ref = create_section_animation();

    let tech_items = mock_tech_data();
    let languages = tech_items
        .iter()
        .filter(|item| matches!(item.tech_type, TechType::ProgrammingLanguage))
        .cloned()
        .collect::<Vec<_>>();

    let (languages, _set_languages) = create_signal(languages);

    let technologies = tech_items
        .iter()
        .filter(|item| matches!(item.tech_type, TechType::Technology))
        .cloned()
        .collect::<Vec<_>>();

    let (technologies, _set_technologies) = create_signal(technologies);

    view! { class = styles,
        <TitleHeader/>
        <main>
            <section node_ref=_who_am_i_ref class="hidden">
                <h2>"Who am I?"</h2>
                <p>
                    " I'm a software engineer with passion for building things.
                        Currently I'm working at "
                    <a href="https://astek.pl" target="_blank">"Astek Polska"</a>
                    " as a full-stack developer
                        studying Computer Science at "
                    <a href="https://cdv.pl" target="_blank">"Collegium Da Vinci"</a>
                    " in Poznań, although I have "
                    {years_str}
                    " of experience in the industry. I'm a constant learner and
                        I'm always trying to improve my skills. I'm brought to
                        the programming world by my passion for video games, so
                        it's no surprise that I'm also a hobbyist game
                        developer. (Well, I'm trying to be) I absolutely love
                        Vulkan, Rust and C++, but I'm pretty much capable of
                        working with whatever needed, because of my genuine
                        curiosity in technologies everall, and anything computer
                        science related can be of interest to me "
                </p>
                <p>
                    " I'm also a huge fan of Linux and open source software. I'm
                        using Linux as my main OS for the last 5 years and I'm
                        not going back. I'm using Arch, btw. "
                </p>
                <p>
                    "Also, sometimes I "
                    <a href="https://www.twitch.tv/clueless_game_dev" target="_blank">
                        "stream"
                    </a>"."
                </p>
            </section>
            <section node_ref=_what_do_i_do_ref class="hidden">
                <h2>"What do I do?"</h2>
                <p>
                    " I'm a web full-stack developer, and I'm trying to learn
                        about programming and Computer Science as much as I can.
                        On spare time I'm playing guitar, video games, and if
                        I'm particularly energized, I'm working on my own
                        projects, web dev and what not. For example for the last
                        few months I've been working on a game engine completely
                        from scratch. I'm also a big fan of Rust and I'm trying
                        to use it whenever I can, so that engine I've mentioned
                        is written in Rust. And" 
                        <a href="https://github.com/nightingazer/personal-page" target="_blank">
                            "this website too. "
                        </a>
             </p>
            </section>
            // <section node_ref=_skillset_ref class="hidden">
            //     <h2>"What is my skillset?"</h2>
            //     <p>
            //         " I'm skilled in a lot of things. From scripting languages
            //             with dynamic types like JavaScript or Python to more
            //             strict C++ and Rust. "
            //     </p>
            //     <CardSwitcher sections={skillset} />
            // </section>
            <section node_ref=_my_last_jobs_ref class="hidden">
                <h2>"Where have I worked recently?"</h2>
                <div class="job-list">
                    {jobs}
                </div>
            </section>
            <section node_ref=_cool_stuff_ref class="hidden">
                <h2>"But what about the cool stuff?"</h2>
                <p>
                    " I have a couple of interesting (for me) projects that I've
                        worked on. Can't really say if that counts as \"cool
                        stuff\" though, but anyway I'm somewhat proud of them as
                        they show me how much I've learned over the years. So
                        here are some of them: "
                </p>
            </section>
            <section node_ref=_be_rust_ref class="hidden">
                <h3>
                    <a href="https://github.com/nightingazer/bizarre-engine-rust" target="_blank">
                        "Bizarre Engine"
                    </a>
                </h3>
                <p>

                    " A game engine I'm working on. It's written in Rust and
                        uses Vulkan for rendering. I'm trying to make it as much
                        from scratch as possible and reasonable. Also, it's
                        worth mentioning that I'm not doing a general-purpose
                        engine, but rather a very specific one. For a game I
                        have in mind. Definitely not a production-ready engine,
                        but it's still a great learning experience and a lot of
                        fun. "
                </p>
                <p>
                    r#"
                        It has Linux as a target platform, but I definitely will maintain Windows support
                        as well. I'm don't hate Windows users, I'm just a Linux fanboy.
                    "#
                </p>
                <p>
                    "It's open source and you can find it on "
                    <a href="https://github.com/nightingazer/bizarre-engine-rust" target="_blank">"my Github"</a>". "
                    "It's open source and it always will be."
                </p>
            </section>
            <section node_ref=_ray_man_ref class="hidden">
                <h3>
                    <a href="https://github.com/nightingazer/RayMan" target="_blank">
                        "RayMan"
                    </a>
                </h3>
                <p>
                    "A funny little software raytracer I've made. It's written
                    in C++ uses ImGui for UI and Walnut as a template for the
                    project. It's not anything special, but it was fun to make
                    and pretty interesting to see how raytracing works. "
                </p>
            </section>
            <section node_ref=_be_cpp_ref class="hidden">
                <h3>
                    <a href="https://github.com/Bizarre-Bits/AGE" targe="_blank">
                        "Another Game Engine (abandoned C++ version)"
                    </a>
                </h3>
                <p>
                    "This is the abandoned C++ version of the engine. It's where
                    I started learning about game engines and graphics
                    programming. And then I've decided to " <i>"rewrite it in
                    Rust™"</i> ". Also, I've got unsatisfied with the design and
                    architecture of the engine, so started anew with the
                    knowledge I've gained. And I thought it would be a good idea to
                    write it in Rust (it is). Also, it uses OpenGL, which is not
                    as fun as Vulkan."
                </p>
            </section>
        </main>


        <footer>
            <p> "Made with " <span class="heart">"❤"</span> " by Alex Hryshan" </p>
            <p> "Version: " <span class="version">{crate_version}</span> </p>
            <p> "This website is open source and you can find it on "
                <a href="https://github.com/nightingazer/personal-page" target="_blank">"Github"</a>". "
                "And huge shout out to "
                <a href="https://leptos.dev" target="_blank">"Leptos Framework"</a>". "
            </p>
            <p>
                "Wow, you've read it all the way to the end. "
                "You're a good reader, I suppose?.."
            </p>
        </footer>
    }
}
