import { Component, OnInit, ViewChild } from '@angular/core';
import { FormGroup, FormBuilder, Validators } from '@angular/forms';
import { MatTable } from '@angular/material';

import User from '@app/types/User';

@Component({
  selector: 'app-home',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.scss']
})
export class HomeComponent implements OnInit {
  @ViewChild(MatTable, { static: true }) table: MatTable<User>;
  columns = ['id', 'name', 'age', 'email'];
  data: User[] = [];

  authForm: FormGroup;

  constructor(private fb: FormBuilder) { }

  ngOnInit() {
    this.authForm = this.fb.group({
      name: ['', [Validators.required]],
      age: ['', [Validators.required]],
      email: ['', [Validators.required, Validators.email]],
    });
  }

  onSubmit() {
    if (this.authForm.valid) {
      this.data.push({ ...this.authForm.value, id: this.data.length + 1 });
      this.table.renderRows();

      this.authForm.reset();
    }
  }

  get name() {
    return this.authForm.get('name');
  }

  get age() {
    return this.authForm.get('age');
  }

  get email() {
    return this.authForm.get('email');
  }
}
