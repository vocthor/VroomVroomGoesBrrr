import { Routes } from '@angular/router';
import {PageNotFoundComponent} from "./components/page-not-found/page-not-found.component";
import {DashboardComponent} from "./components/dashboard/dashboard.component";
import {MapManagerComponent} from "./components/map-manager/map-manager.component";

export const routes: Routes = [
    {
        path: '',
        component: DashboardComponent
    },
    {
        path: 'map',
        component: MapManagerComponent
    },
    {
        path: '**',
        component: PageNotFoundComponent
    }
];
