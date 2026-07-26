import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

ApplicationWindow {
    id: root
    title: "Private Airdrop — Basecamp"
    width: 800
    height: 600
    visible: true

    property string programId: ""
    property string distributionPda: ""
    property string merkleRoot: ""
    property string claimedSoFar: "0"
    property string totalAllocation: "0"
    property bool active: false
    property bool isDistributor: false

    Rectangle {
        anchors.fill: parent
        color: "#1a1a2e"

        ColumnLayout {
            anchors.fill: parent
            anchors.margins: 20
            spacing: 16

            // Header
            Rectangle {
                Layout.fillWidth: true
                height: 60
                color: "#16213e"
                radius: 10

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 15
                    spacing: 10

                    Text {
                        text: "λ Private Airdrop"
                        font.pixelSize: 22
                        font.bold: true
                        color: "#e94560"
                    }

                    Item { Layout.fillWidth: true }

                    Text {
                        text: active ? "● Active" : "○ Inactive"
                        color: active ? "#4ecca3" : "#ff6b6b"
                        font.pixelSize: 14
                    }
                }
            }

            // Main content area
            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                color: "#16213e"
                radius: 10

                RowLayout {
                    anchors.fill: parent
                    anchors.margins: 15
                    spacing: 15

                    // Left Panel: Distribution Info
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: 10

                        Text {
                            text: "Distribution Info"
                            font.pixelSize: 16
                            font.bold: true
                            color: "#ffffff"
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.preferredHeight: 200
                            color: "#0f3460"
                            radius: 8

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 8

                                InfoRow { label: "Program ID"; value: programId.length > 20 ? programId.substring(0, 20) + "..." : "Not set" }
                                InfoRow { label: "Merkle Root"; value: merkleRoot.length > 20 ? merkleRoot.substring(0, 20) + "..." : "Not set" }
                                InfoRow { label: "Claimed"; value: claimedSoFar + " / " + totalAllocation }
                            }
                        }

                        // Action buttons for distributor
                        Rectangle {
                            Layout.fillWidth: true
                            height: isDistributor ? 120 : 0
                            color: "#0f3460"
                            radius: 8
                            visible: isDistributor
                            clip: true

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 8
                                visible: isDistributor

                                Text {
                                    text: "Distributor Actions"
                                    font.pixelSize: 14
                                    color: "#e94560"
                                }

                                Button {
                                    text: "Initialize Distribution"
                                    Layout.fillWidth: true
                                    onClicked: backend.initializeDistribution()
                                }
                                Button {
                                    text: "Close Distribution"
                                    Layout.fillWidth: true
                                    enabled: active
                                    onClicked: backend.closeDistribution()
                                }
                            }
                        }
                    }

                    // Right Panel: Claim
                    ColumnLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        spacing: 10

                        Text {
                            text: "Claim Tokens"
                            font.pixelSize: 16
                            font.bold: true
                            color: "#ffffff"
                        }

                        Rectangle {
                            Layout.fillWidth: true
                            Layout.fillHeight: true
                            color: "#0f3460"
                            radius: 8

                            ColumnLayout {
                                anchors.fill: parent
                                anchors.margins: 12
                                spacing: 8

                                TextField {
                                    id: manifestPathInput
                                    Layout.fillWidth: true
                                    placeholderText: "Path to distribution.json..."
                                    color: "#ffffff"
                                    background: Rectangle {
                                        color: "#1a1a2e"
                                        radius: 4
                                    }
                                }

                                Button {
                                    text: "Check Eligibility"
                                    Layout.fillWidth: true
                                    onClicked: backend.checkEligibility(manifestPathInput.text)
                                }

                                RowLayout {
                                    Layout.fillWidth: true
                                    spacing: 8

                                    Text {
                                        text: "Your allocation:"
                                        color: "#aaaaaa"
                                    }
                                    Text {
                                        id: allocationLabel
                                        text: "—"
                                        color: "#4ecca3"
                                        font.bold: true
                                    }
                                }

                                Button {
                                    text: "Claim My Allocation"
                                    Layout.fillWidth: true
                                    enabled: allocationLabel.text !== "—"
                                    highlighted: true
                                    onClicked: backend.claimTokens(manifestPathInput.text)
                                }

                                Item { Layout.fillHeight: true }

                                Text {
                                    id: statusLabel
                                    text: "Ready"
                                    color: "#888888"
                                    font.pixelSize: 12
                                }
                            }
                        }
                    }
                }
            }

            // Status bar
            Rectangle {
                Layout.fillWidth: true
                height: 30
                color: "#0f3460"
                radius: 5

                Text {
                    anchors.left: parent.left
                    anchors.leftMargin: 10
                    anchors.verticalCenter: parent.verticalCenter
                    text: "Connected to LEZ testnet"
                    color: "#4ecca3"
                    font.pixelSize: 11
                }

                Text {
                    anchors.right: parent.right
                    anchors.rightMargin: 10
                    anchors.verticalCenter: parent.verticalCenter
                    text: "LP-0003"
                    color: "#888888"
                    font.pixelSize: 11
                }
            }
        }
    }
}

component InfoRow: RowLayout {
    property string label: ""
    property string value: ""

    spacing: 8

    Text {
        text: label + ":"
        color: "#aaaaaa"
        font.pixelSize: 12
    }
    Text {
        text: value
        color: "#ffffff"
        font.pixelSize: 12
        elide: Text.ElideRight
    }
}
