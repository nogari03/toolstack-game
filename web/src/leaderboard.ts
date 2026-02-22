export interface GameRecord {
    game: string;
    result: string;
    userId: string;
    score?: number;
    date: string; // ISO String format
}

export class LeaderboardAPI {
    // Using LocalStorage for now, designed as async to simulate real backend API calls later
    static async saveRecord(record: GameRecord): Promise<void> {
        const records = await this.getRecords(record.game);
        records.push(record);

        // Sort logic: if score is present, sort descending, else just show latest
        if (record.score !== undefined) {
            records.sort((a, b) => (b.score || 0) - (a.score || 0));
        } else {
            records.sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime());
        }

        // Keep top 100 max globally
        if (records.length > 100) records.length = 100;

        localStorage.setItem(`leaderboard_${record.game}`, JSON.stringify(records));
    }

    static async getRecords(game: string): Promise<GameRecord[]> {
        const data = localStorage.getItem(`leaderboard_${game}`);
        if (data) {
            try {
                return JSON.parse(data);
            } catch (e) {
                console.error("Failed to parse leaderboard data", e);
                return [];
            }
        }
        return [];
    }
}
