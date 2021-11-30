import { AfterViewInit, Component, Input, OnDestroy, ViewChild } from '@angular/core';
import { MatSort, Sort } from '@angular/material/sort';
import { MatTableDataSource } from '@angular/material/table';
import { dataDir } from '@tauri-apps/api/path';
import { Subscription } from 'rxjs';
import { Tag } from 'src/models/Tag';
import { TagService } from 'src/services/tags.service';

@Component({
    selector: 'e-tags',
    templateUrl: './tags.component.html'
})
export class TagsComponent implements OnDestroy, AfterViewInit{
    sub!: Subscription;
    displayedColumns: string[] = ['id', 'name'];
    dataSource: MatTableDataSource<Tag> = new MatTableDataSource<Tag>();

    @ViewChild(MatSort) sort!: MatSort;

    constructor(public tagService: TagService) {
        this.sub = tagService.view.subscribe(data => this.dataSource.data = data);
        tagService.getTags();
    }

    ngAfterViewInit() {
        this.dataSource.sort = this.sort;
    }

    ngOnDestroy(): void {
        if(this.sub) this.sub.unsubscribe();
    }
}
