import { Component, Input } from '@angular/core';

@Component({
    selector: 'e-header',
    templateUrl: './header.component.html',
    styleUrls: ['./header.component.sass']
})
export class HeaderComponent {
    @Input() height: string = "10%";
    
    public header_buttons = [
        {text: "Button1", fn: () => console.log("button1 pressed")},
        {text: "Button2", fn: () => console.log("button2 pressed")}
    ]
}
