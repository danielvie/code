
export interface TimeMarkerData {
    start: Date,
    finish?: Date,
    duration_ms?: number,
}

export class TimeMarker {
    private markers: { [key: string]: TimeMarkerData}
    
    constructor() {
        this.markers = {}
    }
    
    start(key: string): void {
        if (this.markers[key]) {
            console.log(`Event with key ${key} already exists`)
            return
        }
        
        const start = new Date()
        this.markers[key] = { duration_ms: undefined, start }
    }
    
    stop(key: string): void {
        const time_mark = this.markers[key]
        
        if (time_mark) {
            const finish = new Date()
            const duration_ms = (finish.getTime() - time_mark.start.getTime())
            
            time_mark.finish = finish
            time_mark.duration_ms = duration_ms
        }
    }
    
    get_markers() {
        return this.markers
    }
    
    get_time(label: string) {
        return this.markers[label]
    }
}