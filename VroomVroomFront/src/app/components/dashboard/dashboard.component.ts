import {Component} from '@angular/core';
import {FormsModule} from "@angular/forms";
import {KnobModule} from "primeng/knob";
import {ButtonModule} from 'primeng/button';

@Component({
    selector: 'app-dashboard',
    standalone: true,
    imports: [
        FormsModule,
        KnobModule,
        ButtonModule
    ],
    templateUrl: './dashboard.component.html',
    styleUrls: ['./dashboard.component.css']
})
export class DashboardComponent {
    value: number = 15;
    options: any;

    constructor() {
        this.options = {
            floor: 0,
            ceil: 100
        };
    }

}