pipeline {
    agent { label "rusty" }

    stages {
        stage('Checkout') {
            steps {
                git url: "https://github.com/tpedoussaut/devops-interview-app.git"
            }
        }
        stage('Build') {
            steps {
                sh "/usr/bin/cargo build --release"
            }
        }
        stage('Test') {
            steps {
                sh "/usr/bin/cargo test"
            }
        }
        stage('Docker') {
            steps {
                script {
                    def image = docker.build("tpedoussaut/rusty:latest", ".")
                }    
            }
        }
    }    
 
}
