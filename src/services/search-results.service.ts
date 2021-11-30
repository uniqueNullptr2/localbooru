import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable } from "rxjs";
import { SearchResult } from "src/models/SearchResult";
import { invoke } from '@tauri-apps/api/tauri'

@Injectable({
    providedIn: 'root'
})
export class SearchResultService {

    view: BehaviorSubject<SearchResult[]> = new BehaviorSubject<SearchResult[]>([]);

    public getResults() {
        console.log("starting");
        invoke('search').then(() => {
            console.log("searched yo");
            invoke('get_search_results').then(results => {
                console.log(results as SearchResult[]);
                this.view.next(results as SearchResult[])
            })
        })
    }
}