import { Injectable } from "@angular/core";
import { BehaviorSubject, Observable } from "rxjs";
import { SearchResult } from "src/models/SearchResult";
import { invoke } from '@tauri-apps/api/tauri'
import { CollectionViewer, DataSource } from "@angular/cdk/collections";
import { Tag } from "src/models/Tag";

@Injectable({
    providedIn: 'root'
})
export class TagService{
    view: BehaviorSubject<Tag[]> = new BehaviorSubject<Tag[]>([]);

    tags: Tag[] = [
        new Tag({id: 1, name: "tag1"}),
        new Tag({id: 2, name: "tag2"}),
        new Tag({id: 3, name: "tag3"}),
        new Tag({id: 4, name: "tag4"}),
        new Tag({id: 5, name: "tag5"}),
    ]
    public getTags() {
        this.view.next(this.tags);
    }
}