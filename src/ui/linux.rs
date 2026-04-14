use notify_rust::Notification;


pub fn send_notification(summary: &str, body: &str, appname: &str) {
    Notification::new()
        .summary(summary)
        .body(body)
        .icon("emblem-shared")
        .appname(appname)
        .show()
        .unwrap();
}