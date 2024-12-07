import { Component } from '@angular/core';
import {Router} from "@angular/router";
import {CardModule} from "primeng/card";
import {ButtonDirective} from "primeng/button";

@Component({
  selector: 'app-page-not-found',
  standalone: true,
  imports: [
    CardModule,
    ButtonDirective
  ],
  templateUrl: './page-not-found.component.html',
  styleUrl: './page-not-found.component.css'
})
export class PageNotFoundComponent {
  constructor(private router: Router) {}

  goHome(): void {
    this.router.navigate(['/']);
  }
}
