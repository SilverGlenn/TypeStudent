use crate::state::ActiveView;

#[derive(Clone, Copy, Debug)]
pub struct SidebarItem {
    pub view: ActiveView,
    pub title: &'static str,
    pub icon: &'static str,
    pub badge: Option<&'static str>,
}

pub fn get_sidebar_items() -> Vec<SidebarItem> {
    vec![
        SidebarItem {
            view: ActiveView::CourseOverview,
            title: "Course Lessons",
            icon: "📚",
            badge: Some("12"),
        },
        SidebarItem {
            view: ActiveView::SmartReview,
            title: "Smart Review",
            icon: "🎯",
            badge: None,
        },
        SidebarItem {
            view: ActiveView::StoryStudio,
            title: "Story Studio",
            icon: "📖",
            badge: Some("Stories"),
        },
        SidebarItem {
            view: ActiveView::Trophies,
            title: "Trophy Room",
            icon: "🏆",
            badge: Some("Badges"),
        },
        SidebarItem {
            view: ActiveView::TypingTests,
            title: "Typing Tests",
            icon: "⏱️",
            badge: Some("Diplomas"),
        },
        SidebarItem {
            view: ActiveView::GamesHub,
            title: "Typing Games",
            icon: "🎮",
            badge: Some("4 Games"),
        },
        SidebarItem {
            view: ActiveView::Statistics,
            title: "Statistics & Heatmap",
            icon: "📊",
            badge: None,
        },
        SidebarItem {
            view: ActiveView::Profiles,
            title: "Student Profiles",
            icon: "👥",
            badge: None,
        },
        SidebarItem {
            view: ActiveView::Settings,
            title: "Settings",
            icon: "⚙️",
            badge: None,
        },
    ]
}
