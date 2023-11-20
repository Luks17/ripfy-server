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

  }
}
