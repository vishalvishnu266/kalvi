import React from 'react';
import { BrowserRouter, Routes, Route, Navigate, NavLink } from 'react-router-dom';
import { LayoutDashboard, Users, Settings } from 'lucide-react';
import StudentList from './components/StudentList';
import StudentForm from './components/StudentForm';
import Dashboard from './components/Dashboard';

function App() {
    return (
        <BrowserRouter>
            <div className="d-flex min-vh-100 bg-light">
                {/* Sidebar */}
                <aside className="bg-white border-end shadow-sm" style={{ width: '260px' }}>
                    <div className="p-4 border-bottom">
                        <h4 className="fw-bold mb-0 text-primary d-flex align-items-center">
                            School<span className="text-dark">Desk</span>
                        </h4>
                        <small className="text-muted fw-medium">EDUCATIONAL ERP</small>
                    </div>
                    <nav className="p-3">
                        <ul className="nav nav-pills flex-column gap-1">
                            <li className="nav-item">
                                <NavLink to="/dashboard" className={({ isActive }) => `nav-link d-flex align-items-center gap-3 py-2 px-3 ${isActive ? 'active shadow-sm' : 'text-dark'}`}>
                                    <LayoutDashboard size={20} />
                                    <span>Dashboard</span>
                                </NavLink>
                            </li>
                            <li className="nav-item">
                                <NavLink to="/students" className={({ isActive }) => `nav-link d-flex align-items-center gap-3 py-2 px-3 ${isActive ? 'active shadow-sm' : 'text-dark'}`}>
                                    <Users size={20} />
                                    <span>Students</span>
                                </NavLink>
                            </li>
                            <li className="nav-item mt-4 pt-4 border-top">
                                <NavLink to="/settings" className={({ isActive }) => `nav-link d-flex align-items-center gap-3 py-2 px-3 ${isActive ? 'active shadow-sm' : 'text-dark'}`}>
                                    <Settings size={20} />
                                    <span>Settings</span>
                                </NavLink>
                            </li>
                        </ul>
                    </nav>
                </aside>

                {/* Main Content */}
                <div className="flex-grow-1 overflow-auto">
                    <header className="bg-white border-bottom py-3 px-5 d-flex justify-content-between align-items-center sticky-top shadow-sm">
                        <h5 className="mb-0 fw-bold">Overview</h5>
                        <div className="d-flex align-items-center gap-3">
                            <span className="badge bg-primary-subtle text-primary px-3 py-2">Tenant: School_01</span>
                            <div className="rounded-circle bg-secondary bg-opacity-10 p-2" style={{ width: '38px', height: '38px' }}>
                                <Users size={22} className="text-secondary" />
                            </div>
                        </div>
                    </header>

                    <main className="p-5">
                        <Routes>
                            <Route path="/" element={<Navigate to="/dashboard" replace />} />
                            <Route path="/dashboard" element={<Dashboard />} />
                            <Route path="/students" element={<StudentList />} />
                            <Route path="/students/new" element={<StudentForm />} />
                            <Route path="/students/edit/:id" element={<StudentForm />} />
                            <Route path="/settings" element={<div className="text-center py-5">Settings coming soon...</div>} />
                        </Routes>
                    </main>
                </div>
            </div>
        </BrowserRouter>
    );
}

export default App;
