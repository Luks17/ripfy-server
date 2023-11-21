pipeline {
  agent {
    dockerfile true
  }
  stages {
    stage('Checkout Code') {
      steps {
        git(url: 'https://github.com/Luks17/ripfy-server', branch: 'main')
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build'
      }
    }
    stage('Run migrations') {
      steps {
        sh 'cargo run --bin migrator' 
      }
    }
    stage('Unit & Integration testes') {
      steps {
        sh 'cargo test'
      }
    }

  }
}
