/// Define a trait/"Interface" Colaborador
trait Colaborador {
    fn obtem_salario(&self) -> f32;
    fn carga_horaria(&self) -> u32;
    fn nome(&self) -> String;
}

/// Constante valor base da hora
const VALOR_BASE: f32 = 20.0;

/// Define uma struct Estagiario
struct Estagiario {
    nome: String,
    carga_horaria: u32,
}

impl Estagiario {
    /// Função para criar um Colaborador estagiário
    fn new(nome: &str, carga_horaria: u32) -> Self {
        Estagiario {
            nome: nome.to_string(),
            carga_horaria,
        }
    }
}

/// Define os métodos para estagiários que fazem parte da "Interface" Colaborador
impl Colaborador for Estagiario {
    /// Retorna o valor do salário do estagiário
    fn obtem_salario(&self) -> f32 {
        self.carga_horaria as f32 * VALOR_BASE
    }

    /// Retorna a carga horária
    fn carga_horaria(&self) -> u32 {
        self.carga_horaria
    }

    /// Retorna o nome
    fn nome(&self) -> String {
        format!("{}", self.nome)
    }
}

/// Define uma struct Funcionario
struct Funcionario {
    nome: String,
    carga_horaria: u32,
    nivel: Nivel,
    graduacao: Graduacao,
}

/// Define um Enum para o Nivel de ensino do funcionario(Se professor)
#[derive(PartialEq)]
enum Nivel {
    None,
    Tecnico,
    Graduacao,
    PosGraduacao,
}

/// Define um Enum para o Nivel de graduação do funcionario(Se professor)
enum Graduacao {
    None,
    Graduado,
    Mestrado,
    Doutorado,
}

impl Funcionario {
    fn new(nome: &str, nivel: Nivel, graduacao: Graduacao, carga_horaria: u32) -> Self {
        Funcionario {
            nome: nome.to_string(),
            carga_horaria,
            nivel,
            graduacao,
        }
    }
}

impl Colaborador for Funcionario {
    /// Função para retornar o valor do salário total do funcionario, sendo ele qualquer tipo de funcionario (Nivel -> Define A função do funcionário/professor)
    fn obtem_salario(&self) -> f32 {
        let multiplicador_nivel = match self.nivel {
            Nivel::Tecnico => 3.0,
            Nivel::Graduacao => 4.0,
            Nivel::PosGraduacao => 5.0,
            _ => 2.0, // Se for None, não há multiplicador
        };

        let mut salario_base = self.carga_horaria as f32 * VALOR_BASE * multiplicador_nivel;

        if self.nivel != Nivel::None {
            let adicional_titulacao = match self.graduacao {
                Graduacao::Graduado => 1.0,
                Graduacao::Mestrado => 1.2,
                Graduacao::Doutorado => 1.5,
                _ => 1.0, // Se for None, não há adicional
            };

            salario_base *= adicional_titulacao;
        }

        salario_base
    }

    /// Retorna a carga horária
    fn carga_horaria(&self) -> u32 {
        self.carga_horaria
    }

    /// Retorna o nome
    fn nome(&self) -> String {
        format!("{}*", self.nome)
    }
}

fn main() {

    // Criando uma variável mutavel e adicionando alguns colaboradores
    let mut colaboradores: Vec<Box<dyn Colaborador>> = vec![
        Box::new(Estagiario::new("Eu", 30)),
        Box::new(Funcionario::new("João", Nivel::None, Graduacao::None, 90)),
        Box::new(Funcionario::new(
            "Pedro",
            Nivel::Tecnico,
            Graduacao::Graduado,
            100,
        )),
        Box::new(Funcionario::new(
            "Antônio",
            Nivel::PosGraduacao,
            Graduacao::Doutorado,
            200,
        )),
        Box::new(Estagiario::new("José", 35)),
        Box::new(Estagiario::new("Christofer", 20)),
    ];
    
    // Adicionando mais um colaborador
    colaboradores.push(Box::new(Funcionario::new(
        "Daniel",
        Nivel::Graduacao,
        Graduacao::Mestrado,
        90,
    )));

    // Printando o nome dos colaboradores e salários
    for colaborador in colaboradores.iter() {
        println!("{}: R${}", colaborador.nome(), colaborador.obtem_salario());
    }
}
