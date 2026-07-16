export interface Student {
    id: string;
    admission_no: string;
    first_name: string;
    last_name: string;
    email?: string;
    phone?: string;
    date_of_birth?: string;
    gender: string;
    blood_group?: string;
    class_name: string;
    section: string;
    roll_no: string;
    admission_date: string;
    guardian_name: string;
    guardian_phone: string;
    guardian_email?: string;
    guardian_relation?: string;
    address_line?: string;
    city?: string;
    state?: string;
    postal_code?: string;
    status: string;
    created_at?: string;
    updated_at?: string;
}

export interface StudentFilters {
    q?: string;
    class_name?: string;
    section?: string;
    status?: string;
}

export interface DashboardStats {
    total_students: number;
    active_students: number;
    pending_students: number;
    inactive_students: number;
    recent_students: Student[];
}

export interface ApiError {
    status: number;
    message: string;
    error_type?: string;
}
