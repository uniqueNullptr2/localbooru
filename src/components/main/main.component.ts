import { Component } from '@angular/core';
import { Observable } from 'rxjs';
import { SearchResult } from 'src/models/SearchResult';
import { SearchResultService } from 'src/services/search-results.service';

@Component({
    selector: 'zmain',
    templateUrl: './main.component.html',
    styleUrls: ['./main.component.sass']
})
export class MainComponent {
    public view: Observable<SearchResult[]>;
    constructor(private resultService: SearchResultService) {
        this.view = resultService.view;
        resultService.getResults();
    }
}
