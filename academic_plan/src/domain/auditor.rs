use crate::{domain::Class, raw::row::RawRow};

#[derive(Debug)]
pub enum IssueSeverity {
    Warning,
    Critical,
}

#[derive(Debug)]
pub struct AuditIssue {
    pub course_code: String,
    pub field: String,
    pub expected: String,
    pub found: String,
    pub severity: IssueSeverity,
}

pub struct AuditReport {
    pub total_processed: usize,
    pub issues: Vec<AuditIssue>,
    pub critical_count: usize,
}

impl AuditReport {
    pub fn accuracy_score(&self) -> f32 {
        if self.total_processed == 0 { return 0.0; }
        100.0 - (self.critical_count as f32 / self.total_processed as f32 * 100.0)
    }

    pub fn print_summary(&self) {
        println!("\n--- 📊 REPORTE DE AUDITORÍA ---");
        for issue in &self.issues {
            let label = match issue.severity {
                IssueSeverity::Critical => "❌ [CRÍTICO]",
                IssueSeverity::Warning => "⚠️ [AVISO]",
            };
            println!("{} Asignatura {}: En '{}' se esperaba '{}' pero se halló '{}'", 
                label, issue.course_code, issue.field, issue.expected, issue.found);
        }
        println!("\nTotal filas: {}", self.total_processed);
        println!("Errores críticos: {}", self.critical_count);
        println!("Puntaje de precisión: {:.2}%", self.accuracy_score());
    }
}

pub struct PlanAuditor;

impl PlanAuditor {
    /// Punto de entrada principal para auditar un dataset completo
    pub fn run(rows: &[RawRow]) -> AuditReport {
        let mut report = AuditReport {
            total_processed: rows.len(),
            issues: Vec::new(),
            critical_count: 0,
        };

        for row in rows {
            let mut row_issues = Self::audit_row(row);
            
            // Contabilizar críticos
            report.critical_count += row_issues.iter()
                .filter(|i| matches!(i.severity, IssueSeverity::Critical))
                .count();
                
            report.issues.append(&mut row_issues);
        }

        report
    }

    fn audit_row(row: &RawRow) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        // 1. Regla Aritmética: HT + HP + HTI == TH
        if let Some(issue) = Self::verify_arithmetic(row) {
            issues.push(issue);
        }

        // 2. Regla de Créditos: TC debe ser razonable
        if let Some(issue) = Self::verify_credits(row) {
            issues.push(issue);
        }

        // 3. Regla de Truncado: Verificar fin de cadenas
        if let Some(issue) = Self::verify_naming_integrity(row) {
            issues.push(issue);
        }

        issues
    }

    fn verify_arithmetic(row: &RawRow) -> Option<AuditIssue> {
        let ht = row.teorical_hours.parse::<u32>().unwrap_or(0);
        let hp = row.practical_hours.parse::<u32>().unwrap_or(0);
        let hti = row.teorical_practical_hours.parse::<u32>().unwrap_or(0);
        let th = row.total_hours.parse::<u32>().unwrap_or(0);

        if ht + hp + hti != th {
            return Some(AuditIssue {
                course_code: row.course_code.clone(),
                field: "TH (Total Hours)".to_string(),
                expected: (ht + hp + hti).to_string(),
                found: th.to_string(),
                severity: IssueSeverity::Critical,
            });
        }
        None
    }

    fn verify_credits(row: &RawRow) -> Option<AuditIssue> {
        let tc = row.total_credits.parse::<u32>().unwrap_or(0);
        if tc == 0 || tc > 6 {
            return Some(AuditIssue {
                course_code: row.course_code.clone(),
                field: "TC (Créditos)".to_string(),
                expected: "2-5".to_string(),
                found: tc.to_string(),
                severity: IssueSeverity::Critical,
            });
        }
        None
    }

    fn verify_naming_integrity(row: &RawRow) -> Option<AuditIssue> {
    let prepositions = ["de", "la", "y", "a", "con", "en"];
    let name = row.course_name.trim();
    let words: Vec<&str> = name.split_whitespace().collect();
    
    if let Some(last_word) = words.last() {
        // Solo es aviso si termina en preposición en minúscula
        if prepositions.contains(&last_word.to_lowercase().as_str()) {
            return Some(AuditIssue {
                course_code: row.course_code.clone(),
                field: "course_name".to_string(),
                expected: "Nombre completo".to_string(),
                found: row.course_name.clone(),
                severity: IssueSeverity::Warning,
            });
        }
    }
    None
}

pub fn run_final(classes: &[Class]) -> AuditReport {
        let mut report = AuditReport {
            total_processed: classes.len(),
            issues: Vec::new(),
            critical_count: 0,
        };

        for class in classes {
            let mut class_issues = Self::audit_class(class);
            
            report.critical_count += class_issues.iter()
                .filter(|i| matches!(i.severity, IssueSeverity::Critical))
                .count();
                
            report.issues.append(&mut class_issues);
        }

        report
    }

    /// Lógica de auditoría adaptada a la estructura Class
    fn audit_class(class: &Class) -> Vec<AuditIssue> {
        let mut issues = Vec::new();

        // 1. Regla Aritmética: HT + HP + HTI == TH
        // Nota: Aquí ya usamos los campos numéricos directamente
        if class.theoretical_hours + class.practical_hours + class.independent_hours != class.total_hours {
            issues.push(AuditIssue {
                course_code: class.code.clone(),
                field: "TH (Total Hours)".to_string(),
                expected: (class.theoretical_hours + class.practical_hours + class.independent_hours).to_string(),
                found: class.total_hours.to_string(),
                severity: IssueSeverity::Critical,
            });
        }

        // 2. Regla de Créditos
        if class.credits == 0 || class.credits > 6 {
            issues.push(AuditIssue {
                course_code: class.code.clone(),
                field: "TC (Créditos)".to_string(),
                expected: "2-5".to_string(),
                found: class.credits.to_string(),
                severity: IssueSeverity::Critical,
            });
        }

        // 3. Regla de Truncado
        if let Some(issue) = Self::verify_naming_integrity_class(class) {
            issues.push(issue);
        }

        issues
    }

    fn verify_naming_integrity_class(class: &Class) -> Option<AuditIssue> {
        let prepositions = ["de", "la", "y", "a", "con", "en"];
        let words: Vec<&str> = class.name.split_whitespace().collect();
        
        if let Some(last_word) = words.last() {
            if prepositions.contains(&last_word.to_lowercase().as_str()) {
                return Some(AuditIssue {
                    course_code: class.code.clone(),
                    field: "course_name".to_string(),
                    expected: "Nombre completo".to_string(),
                    found: class.name.clone(),
                    severity: IssueSeverity::Warning,
                });
            }
        }
        None
    }
}