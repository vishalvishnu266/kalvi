import React from 'react';
import { useQuery } from '@tanstack/react-query';
import { StudentApi } from '../api/students';
import { Users, UserCheck, UserPlus, UserX, Clock } from 'lucide-react';
import { useNavigate } from 'react-router-dom';

const Dashboard = () => {
    const navigate = useNavigate();
    const { data: stats, isLoading, error } = useQuery({
        queryKey: ['dashboard-stats'],
        queryFn: StudentApi.getDashboardStats
    });

    if (isLoading) return <div className="text-center py-5">Loading dashboard...</div>;
    if (error) return <div className="alert alert-danger">Error loading stats</div>;

    const cards = [
        { title: 'Total Students', value: stats?.total_students, icon: <Users size={24} />, color: 'primary' },
        { title: 'Active', value: stats?.active_students, icon: <UserCheck size={24} />, color: 'success' },
        { title: 'Pending', value: stats?.pending_students, icon: <UserPlus size={24} />, color: 'warning' },
        { title: 'Inactive', value: stats?.inactive_students, icon: <UserX size={24} />, color: 'danger' },
    ];

    return (
        <div>
            <div className="row g-4 mb-5">
                {cards.map((card, i) => (
                    <div key={i} className="col-md-3">
                        <div className={`card border-0 shadow-sm border-start border-4 border-${card.color}`}>
                            <div className="card-body p-4">
                                <div className="d-flex justify-content-between align-items-center">
                                    <div>
                                        <p className="text-muted mb-1 fw-medium">{card.title}</p>
                                        <h3 className="mb-0 fw-bold">{card.value}</h3>
                                    </div>
                                    <div className={`p-3 bg-${card.color} bg-opacity-10 text-${card.color} rounded-circle`}>
                                        {card.icon}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            <div className="card shadow-sm border-0 mb-4">
                <div className="card-header bg-white py-3">
                    <h5 className="mb-0 fw-bold text-dark">Recent Enrollments</h5>
                </div>
                <div className="card-body p-0">
                    <div className="table-responsive">
                        <table className="table table-hover align-middle mb-0">
                            <thead className="table-light">
                                <tr>
                                    <th>Student</th>
                                    <th>Class</th>
                                    <th>Admission Date</th>
                                    <th className="text-end">Action</th>
                                </tr>
                            </thead>
                            <tbody>
                                {stats?.recent_students.map(student => (
                                    <tr key={student.id}>
                                        <td>
                                            <div className="fw-bold">{student.first_name} {student.last_name}</div>
                                            <div className="text-muted small">{student.admission_no}</div>
                                        </td>
                                        <td>{student.class_name}-{student.section}</td>
                                        <td>
                                            <div className="d-flex align-items-center small text-muted">
                                                <Clock size={14} className="me-1" />
                                                {student.admission_date}
                                            </div>
                                        </td>
                                        <td className="text-end">
                                            <button 
                                                className="btn btn-outline-primary btn-sm"
                                                onClick={() => navigate(`/students/edit/${student.id}`)}
                                            >
                                                View
                                            </button>
                                        </td>
                                    </tr>
                                ))}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default Dashboard;
