import { Component, ViewChild } from '@angular/core';
import { MatSelectionList } from '@angular/material/list';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.sass']
})
export class AppComponent {
  title = 'localbooru';
  time = new Date().toISOString();

  @ViewChild("#selectionList") selectionList !: MatSelectionList;
  constructor() {
    setInterval(() => this.time = new Date().toISOString(), 100)
  }

  public tagList: string[]=[
    "tag",
    "tag2",
    "tag3 electric boogaloo"
  ]
}
