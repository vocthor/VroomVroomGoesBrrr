import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  imports: [
    RouterOutlet
  ],
  standalone: true,
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'VroomVroomFront';
}
