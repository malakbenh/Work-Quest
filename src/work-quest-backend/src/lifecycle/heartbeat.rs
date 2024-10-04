use crate::RUNTIME_STATE;
use ic_cdk_macros::heartbeat;
use ic_cdk::println;

#[heartbeat]
async fn heartbeat() {
    RUNTIME_STATE.with(|state| {
        let mut state = state.borrow_mut();

        // Log the heartbeat execution for monitoring
        println!("Heartbeat triggered");

        // Task 1: Clean up expired jobs
        if let Err(e) = state.data.clean_up_expired_jobs() {
            println!("Error cleaning up expired jobs: {}", e);
        } else {
            println!("Successfully cleaned up expired jobs");
        }

        // Task 2: Send pending notifications
        if let Err(e) = state.data.send_notifications() {
            println!("Error sending notifications: {}", e);
        } else {
            println!("Successfully sent notifications");
        }

        // Task 3: Ledger sync - Check for new transactions and import them if necessary
        let result = ic_cdk::spawn(async {
            let ledger_result = state.ledger.get_blocks_since(state.data.last_imported_block, 100).await;
            match ledger_result {
                Ok(blocks) => {
                    state.data.process_new_blocks(blocks);
                    println!("Ledger synced successfully");
                },
                Err(e) => println!("Error syncing ledger: {}", e),
            }
        });

        // Task 4: Periodic analytics (optional)
        if let Err(e) = state.data.generate_analytics_report() {
            println!("Error generating analytics report: {}", e);
        } else {
            println!("Analytics report generated");
        }

        // Additional periodic tasks can be added as needed
    });
}
