fn on_key_event(event: TypingEvent, monitor: &mut BehavioralMonitor) {
    monitor.record_event(event);

    if !monitor.evaluate() {
        // Trigger session lock and prompt re-authentication
        SessionManager::lock();
        UI::show_lock_screen();
    }
}
