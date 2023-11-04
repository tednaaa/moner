import { NgModule } from "@angular/core";
import { RouterModule, Routes } from "@angular/router";

import { routes } from "@/shared/routes";
import { HomeComponent } from "@/pages/home/home.component";

const router: Routes = [{ path: routes.HOME, component: HomeComponent }];

@NgModule({
  imports: [RouterModule.forRoot(router)],
  exports: [RouterModule],
})
export class AppRoutingModule {}
